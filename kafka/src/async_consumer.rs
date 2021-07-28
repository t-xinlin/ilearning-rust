use crate::config;
use crate::util;

use futures::stream::FuturesUnordered;
use futures::{StreamExt, TryStreamExt};
use log::*;
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::{BorrowedMessage, OwnedMessage};
use rdkafka::{ClientContext, Message, Statistics, TopicPartitionList};

pub async fn async_consumer() {
    info!("kafka consumer run");
    let num_workers: usize = 1;
    (0..num_workers)
        .map(|_| tokio::spawn(run_async_processor()))
        .collect::<FuturesUnordered<_>>()
        .for_each(|_| async move { () })
        .await
}

async fn run_async_processor() {
    run_consumer().await;
}

pub async fn run_consumer() {
    let (version_n, version_s) = rdkafka::util::get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    let kafka_config = config::init_kafka();
    let topics_config = kafka_config.topics.unwrap();
    let topics: Vec<&str> = topics_config.iter().map(|s| s.as_str()).collect();
    let consumer: StreamConsumer = util::client_config(Some(kafka_config.config))
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        // .create_with_context(context)
        .expect("Consumer creation failed");

    consumer
        .subscribe(topics.as_slice())
        .expect("Can't subscribe to specified topic");

    let stream_processor = consumer.stream().try_for_each(|msg| async move {
        record_borrowed_message_receipt(&msg).await;
        let owned_message = msg.detach();
        record_owned_message_receipt(&owned_message).await;
        Ok(())
    });
    info!("starting event loop");
    stream_processor.await.expect("stream processing failed");
    info!("stream processing terminated");
}

async fn record_borrowed_message_receipt(msg: &BorrowedMessage<'_>) {
    let payload = match msg.payload() {
        None => {
            let err_msg = String::from("Error while deserializing message payload");
            warn!("{}", err_msg);
            err_msg
        }
        Some(b) => {
            // let dep = util::frame_depress(b);
            let dep = match util::decompress(b) {
                Ok(o) => o,
                Err(e) => {
                    let err_msg = String::from("error depress message payload");
                    error!("{:?}", err_msg);
                    e.to_string()
                }
            };
            dep
        }
    };

    let k = match msg.key() {
        None => {
            let err_key = String::from("key: Error while deserializing message payload");
            warn!("{}", err_key);
            err_key
        }
        Some(b) => util::to_string(b),
    };

    info!(
        "consumer key:{:?}, payload:{:?}, topic:{}, partition:{}, offset:{}",
        k,
        payload,
        msg.topic(),
        msg.partition(),
        msg.offset()
    );

    // info!("======={}", msg.h)

    if let Some(_h) = msg.headers() {
        info!("========get header ok=========")
        // for i in 0..h.count() {
        //     let header = h.get(i).unwrap();
        //     info!("header {:#?}: {:?}", header.0, header.1);
        // }
    }
    // consumer.commit_message(&msg, CommitMode::Async).unwrap();
}

async fn record_owned_message_receipt(_msg: &OwnedMessage) {
    // Like `record_borrowed_message_receipt`, but takes an `OwnedMessage`
    // instead, as in a real-world use case  an `OwnedMessage` might be more
    // convenient than a `BorrowedMessage`.
}

struct CustomContext;

impl ClientContext for CustomContext {
    // Access stats
    fn stats(&self, stats: Statistics) {
        let stats_str = format!("{:?}", stats);
        info!("Stats received: {} bytes", stats_str.len());
    }
}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        info!("Committing offsets: {:?}", result);
    }
}
