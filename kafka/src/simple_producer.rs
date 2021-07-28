use crate::config;
use crate::util;
use log::info;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

pub async fn simple_producer() {
    let (version_n, version_s) = rdkafka::util::get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);
    produce().await;
}

async fn produce() {
    let kafka_config = config::init_kafka();
    let output_topic = kafka_config.topic.unwrap();
    let topic = output_topic.as_str();
    let producer: &FutureProducer = &util::client_config(Some(kafka_config.config))
        .create()
        .expect("Producer creation error");

    let futures = (0..10000)
        .map(|i| async move {
            let delivery_status = producer
                .send(
                    FutureRecord::to(topic)
                        .payload(&format!("Message {}", i))
                        .key(&format!("Key {}", i))
                        .headers(OwnedHeaders::new().add("header_key", "header_value")),
                    Duration::from_secs(0),
                )
                .await;

            info!("Delivery status for message {} received", i);
            delivery_status
        })
        .collect::<Vec<_>>();

    for future in futures {
        info!("Future completed. Result: {:?}", future.await);
    }
}
