use actix_web::{error, get, post, web, App, HttpServer, Responder, HttpRequest, HttpResponse, Result};
use std::sync::Mutex;
use serde::{Deserialize, Serialize};


// #[derive(Deserialize)]
#[derive(Serialize, Deserialize)]
struct Info {
    username: String,
}

/// deserialize `Info` from request's body
#[post("/user")]
pub async fn userHandler(info: web::Json<Info>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(Info {
        username: info.username.to_string(),
    }))
}
