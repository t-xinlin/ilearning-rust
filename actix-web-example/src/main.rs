#[macro_use]
extern crate log;
extern crate log4rs;
extern crate lazy_static;

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
pub mod model;

use crate::utils::{
    log as sys_log,
    scheduler,
    scheduler::JobTrait,
    counter::Iterator,
    counter,
    // signal,
};
use crate::conf::config;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // init log
    sys_log::init().unwrap();
    counter().await;
    // init db
    config::init_db().await;
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
    let conf = config::GLOBAL_CONFIG.lock().unwrap();
    info!("GLOBAL_CONFIG: {:?}",  conf.clone());
    let counter = basic::new_counter();
    let mut app = HttpServer::new(move || App::new()
        .wrap(Cors::default()
                  .allow_any_origin() // .allowed_origin("*")
                  .allowed_methods(vec!["GET", "POST", "DELETE"])
                  .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                  .allowed_header(header::CONTENT_TYPE)
                  .allowed_header("sign")
                  .supports_credentials()
                  .max_age(3600),
        )
        .wrap(middleware::AccessLogging::default().log_target("http_log"))
        .app_data(counter.clone()) // <- register the created data
        .configure(routes)
    );

    for i in conf.clone().server.services.unwrap() {
        app = app.bind(format!("{}:{}", i.address.unwrap(), i.port.unwrap())).unwrap();
    }
    app.run().await
}

async fn scheduler_job() {
    let scheduler_expr: &str = "1/10 * * * * *";
    // job run
    let scheduler_job = scheduler::SchedulerJob::new().expect("scheduler job run");
    scheduler_job.run(scheduler_expr).await.expect("scheduler job run error")
}

async fn counter() {
    let mut c = counter::Counter::new();
    let c1 = c.next().unwrap();
    println!("c1={}", c1);
}
