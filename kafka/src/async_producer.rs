use crate::config;
use crate::util;
use log::*;
use rdkafka::message::{OwnedHeaders, ToBytes};
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;
use tokio::time::Instant;

pub async fn run_producer() {
    info!("producer run");
    let (version_n, version_s) = rdkafka::util::get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    let mut kafka_config = config::init_kafka();
    let producer: FutureProducer = util::client_config(Some(kafka_config.config))
        .create()
        .expect("producer creation error");

    let producer = producer.clone();
    let topic = kafka_config.topic.unwrap();
    let millisecond = 100;
    let mut interval = tokio::time::interval(Duration::from_millis(millisecond));
    let start = Instant::now();
    info!("time:{:?}", start);
    loop {
        interval.tick().await;
        let timestamp = start.elapsed().as_secs_f64();
        // info!("time:{:?}", start.elapsed());
        let msg = &format!("MSG {:?}", start.elapsed());
        info!("producer {}", msg);
        let pre = match util::compressed(msg.to_bytes()) {
            Ok(o) => o,
            Err(e) => {
                error!("{:?}", e);
                continue;
            }
        };

        let headers = OwnedHeaders::new()
            .add("header_key", "header_value")
            .add("header_key01", "header_value1")
            .add("header_key02", "header_value2")
            .add("header_key03", "header_value3")
            .add("header_key04", "header_value4");

        // println!("{}",format!("{:#?}",t.sec));
        let delivery_status = producer.send(
            FutureRecord::to(topic.as_str())
                .payload(&pre)
                .key("key-001")
                .timestamp(timestamp as i64)
                // .headers(headers),
                .headers(headers),
            Duration::from_secs(0),
        );
        match delivery_status.await {
            Ok(_delivery) => {} // info!("producer: {:?}", delivery),
            Err((e, _)) => info!("error: {:?}", e),
        }
    }
}

//
// fn ms(n: u64) -> Duration {
//     Duration::from_millis(n)
// }
