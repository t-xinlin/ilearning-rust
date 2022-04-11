#[macro_use]
extern crate log as other_log;
extern crate log4rs;

use actix_web::{
    web,
    App,
    HttpServer,
};
use actix_web_example::{
    middleware,
    log::log,
    handler::{basic, user},
};
use actix_web_example::router::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log::init().unwrap();
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
    ).bind("127.0.0.1:8080")?.run().await
}


