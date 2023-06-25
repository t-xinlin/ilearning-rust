#[macro_use]
extern crate log as other_log;
extern crate log4rs;

use actix_web::{
    App,
    HttpServer,
};
use actix_web_example::{
    middleware,
    handler::{basic},
    router::routes,
};


pub mod utils;

use std::process;
use tokio::io::AsyncWriteExt;
use crate::utils::log as sys_log;
use crate::utils::scheduler;
use crate::utils::scheduler::JobTrait;
use crate::utils::counter::Iterator;
use crate::utils::counter;

use crate::utils::config;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    // init log
    sys_log::init().unwrap();
    counter().await;
    scheduler_job().await;
    run().await
}

async fn run() -> std::io::Result<()> {
    // log::init().unwrap();
    info!("service starting...");
    let counter = basic::new_counter();
    // scheduler::RunScheduler().await?;
    HttpServer::new(move || App::new()
        // .wrap(middleware::ReadReqBody)
        .wrap(middleware::Jwt)
        // .wrap(middleware::AccessLogging)
        .wrap(middleware::AccessLogging::default().log_target("http_log"))
        .app_data(counter.clone()) // <- register the created data
        .configure(routes)
    ).bind("127.0.0.1:8088").unwrap().run().await
}


async fn scheduler_job() {
    config::init_env();

    let scheduler_expr: &str = "1/3 * * * * *";
    // job run
    let scheduler_job = scheduler::build().unwrap_or_else(|err| {
        error!("scheduler job run: {}", err);
        process::exit(1);
    });
    scheduler_job.run(scheduler_expr).await.unwrap_or_else(|err| {
        error!("scheduler job run: {}", err);
        process::exit(1);
    });
}


async fn counter() {
    let mut c = counter::Counter::new();
    let c1 = c.next().unwrap();
    println!("c1={}", c1);
}
