#[macro_use]
extern crate log as other_log;
extern crate log4rs;

use actix_cors::Cors;
use actix_web::{
    App,
    HttpServer,
    http::header,
};
use actix_web_example::{
    middleware,
    handler::{basic},
    router::routes,
};

pub mod utils;
pub mod conf;

use std::net::UdpSocket;
use crate::utils::log as sys_log;
use crate::utils::scheduler;
use crate::utils::scheduler::JobTrait;
use crate::utils::counter::Iterator;
use crate::utils::counter;
// use crate::utils::signal;
use crate::conf::config;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // init log
    sys_log::init().unwrap();
    // init config
    // let config = config::Conf::new().unwrap_or_else(|err| {
    //     error!("Problem parsing arguments: {:?}", err);
    //     process::exit(1);
    // });
    // let c = config::CACHE.clone();
    info!("config::CACHE.lock(): {:?}",   config::CACHE.clone().lock().unwrap());

    counter().await;
    scheduler_job().await;
    run().await

    // tokio::select! {
    //     _ = signal::shutdown() => {
    //         info!("Received shutdown signal");
    //     }
    //     // _ = shutdown_rx.recv() => {
    //     //     info!("Received shutdown via admin interface");
    //     // }
    // }
    // Ok(())
}

async fn run() -> std::io::Result<()> {
    // log::init().unwrap();
    let local_addr = what_is_my_ip().unwrap();
    info!("service starting on: {}", local_addr);
    let counter = basic::new_counter();
    // scheduler::RunScheduler().await?;
    HttpServer::new(move || App::new()
        .wrap(
            Cors::default()
                .allow_any_origin() // .allowed_origin("*")
                .allowed_methods(vec!["GET", "POST", "DELETE"])
                .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                .allowed_header(header::CONTENT_TYPE)
                .allowed_header("sign")
                .supports_credentials()
                .max_age(3600),
        )
        // .wrap(middleware::ReadReqBody)
        // .wrap(middleware::Jwt)
        // .wrap(middleware::AccessLogging)
        .wrap(middleware::AccessLogging::default().log_target("http_log"))
        .app_data(counter.clone()) // <- register the created data
        .configure(routes)
    ).bind(format!("{}:8088", local_addr)).unwrap().run().await
}

async fn scheduler_job() {
    let scheduler_expr: &str = "1/3 * * * * *";
    // job run
    let scheduler_job = scheduler::build().expect("scheduler job run");
    scheduler_job.run(scheduler_expr).await.expect("scheduler job run error")
}

async fn counter() {
    let mut c = counter::Counter::new();
    let c1 = c.next().unwrap();
    println!("c1={}", c1);
}

//#![windows_subsystem = "windows"]
fn what_is_my_ip() -> Option<String> {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return None,
    };
    match socket.connect("8.8.8.8:80") {
        Ok(()) => (),
        Err(_) => return None,
    };
    match socket.local_addr() {
        Ok(addr) => Some(addr.ip().to_string()),
        Err(_) => return None,
    }
}
