// #[macro_use]
// extern crate log as other_log;
use kafka_v1::async_consumer;
use kafka_v1::async_producer;
use kafka_v1::config;
use kafka_v1::log::*;
use log::*;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    init();
    // tokio::task::spawn(simple_consumer::simple_consumer());
    // tokio::task::spawn(async_producer::run_producer());
    // tokio::task::spawn(async_consumer::async_consumer());
    tokio::task::spawn(async_producer::run_producer());
    tokio::time::sleep(Duration::from_secs(60 * 10)).await;
    info!("main exit");
}

fn init() {
    log_init();
    config::init_config();
    let k = config::init_kafka();
    info!("kafka config: {:?}", k)
}
