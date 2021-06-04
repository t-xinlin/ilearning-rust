#[macro_use]
extern crate log as other_log;
extern crate log4rs;

use actix_web::{error, get, post, web, App, HttpServer, Responder, HttpRequest, HttpResponse, Result};
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use actix_web_example::middleware;
use actix_web_example::log::log;
use actix_web_example::cron;
use actix_web_example::user::user;
use actix_web_example::cron::scheduler;
use actix_web_example::error::user_error;
use actix_web_example::security;
use tokio::time::{sleep, Duration};
use actix_http::http::ContentEncoding;

#[get("/index.html")]
async fn index(req: HttpRequest) -> impl Responder {
    format!("Hello {}!", "index2")
}

// // #[get("/test/{id}/{name}")]
// async fn test(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
//     println!("{}{}", id, name);
//     format!("Hello {}! id:{}", name, id)
// }


async fn greet(req: HttpRequest) -> impl Responder {
    println!("greet req");
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

async fn state(data: web::Data<AppStateWithCounter>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard
    format!("Request number: {}", counter) // <- response with count
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log::init();
    security::hmac::testHmacSha256();
    info!("service starting...");
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });
    // scheduler::RunScheduler().await?;
    HttpServer::new(move || App::new()
        .wrap(middleware::read_request_body::ReadReqBody)
        .wrap(middleware::jwt::Jwt)
        .app_data(counter.clone()) // <- register the created data
        .service(index)
        .service(
            web::scope("/app")
                .route("/greet", web::get().to(greet))
                .route("/state", web::post().to(state))
                .service(user::userHandler),
        )
                    // .service(
                    //     web::scope("/test")
                    //         .route("/{id}/{name}", web::get().to(test))
                    // )
    )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}


