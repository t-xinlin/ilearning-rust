// #[macro_use]
// extern crate log as other_log;
use kafka_v1::async_consumer;
use kafka_v1::async_producer;
use kafka_v1::config;
use kafka_v1::log::*;
use kafka_v1::signal;
use log::*;
// use tokio::signal::unix::SignalKind;
// use tokio::sync::mpsc;
// use tokio::time::Duration;

#[tokio::main]
async fn main() {
    init();
    tokio::task::spawn(async_consumer::async_consumer());
    tokio::task::spawn(async_producer::run_producer());

    // tokio::time::sleep(Duration::from_secs(60 * 10)).await;
    // let (_shutdown_tx, mut shutdown_rx) = mpsc::unbounded_channel();
    tokio::select! {
        _ = signal::shutdown() => {
            info!("Received shutdown signal");
        }
        // _ = shutdown_rx.recv() => {
        //     info!("Received shutdown via admin interface");
        // }
    }
    info!("service shutdown")
}

fn init() {
    log_init();
    // config::init_config();
    let k = config::init_kafka();
    info!("kafka config: {:?}", k)
}
