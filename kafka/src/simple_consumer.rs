use crate::config;
use crate::util;

use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::{Headers, Message};
use rdkafka::topic_partition_list::TopicPartitionList;
use rdkafka::Statistics;

use log::*;
use std::collections::HashMap;

use tokio::sync::mpsc;

pub async fn simple_consumer() {
    info!("kafka consumer run...");
    consume_and_print().await
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

pub fn consumer_config(config_overrides: Option<HashMap<&str, &str>>) -> ClientConfig {
    let mut config = ClientConfig::new();
    if let Some(overrides) = config_overrides {
        for (key, value) in overrides {
            config.set(key, value);
        }
    }
    config
}

async fn consume_and_print() {
    let (version_n, version_s) = rdkafka::util::get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    let context = CustomContext {};
    let kafka_config = config::init_kafka();
    let topics_config = kafka_config.topics.unwrap();
    let topics: Vec<&str> = topics_config.iter().map(|s| s.as_str()).collect();
    let consumer: StreamConsumer<CustomContext> = util::client_config(Some(kafka_config.config))
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");
    consumer
        .subscribe(topics.as_slice())
        .expect("Can't subscribe to specified topics");

    let (tx_in, mut rx_in) = mpsc::channel::<String>(5);

    tokio::task::spawn(async move {
        while let Some(data) = rx_in.recv().await {
            info!("recv: {}", data);
        }
    });

    loop {
        match consumer.recv().await {
            Err(e) => warn!("Kafka error: {}", e),
            Ok(m) => {
                let payload = match m.payload() {
                    None => {
                        let err_msg = String::from("Error while deserializing message payload");
                        warn!("{}", err_msg);
                        err_msg
                    }
                    Some(b) => {
                        // let dep = util::frame_depress(b);
                        let dep = match util::frame_depress(b) {
                            Ok(o) => o,
                            Err(e) => {
                                let err_msg = String::from("error depress message payload");
                                error!("{:?}", err_msg);
                                Vec::from(e.to_string())
                            }
                        };
                        util::escape(&dep)
                    }
                };

                let k = match m.key() {
                    None => {
                        let err_key =
                            String::from("key: Error while deserializing message payload");
                        warn!("{}", err_key);
                        err_key
                    }
                    Some(b) => util::to_string(b),
                };
                info!(
                    "key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}",
                    k,
                    payload,
                    m.topic(),
                    m.partition(),
                    m.offset()
                );
                tx_in.send(payload).await.unwrap();
                if let Some(headers) = m.headers() {
                    for i in 0..headers.count() {
                        let header = headers.get(i).unwrap();
                        info!("  Header {:#?}: {:?}", header.0, header.1);
                    }
                }
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}
