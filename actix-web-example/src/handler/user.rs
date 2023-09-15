use std::collections::HashMap;

use actix_web::{
    error,
    post,
    web,
    Error,
    // Result,
    HttpRequest,
    HttpResponse,
};
use futures_util::StreamExt;
// use json::JsonValue;
use serde::{Deserialize, Serialize};
// use serde_json::value::Value;
use serde_json::{json, Value};

// #[derive(Deserialize)]
#[derive(Serialize, Deserialize)]
pub struct Info {
    username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    code: i32,
    data: Option<HashMap<String, Value>>,
    msg: String,
}

#[post("/json")]
pub async fn json_handler(info: web::Json<UserInfo>) -> HttpResponse {
    HttpResponse::Ok().json(info.0)
}

#[post("/extract_json")]
pub async fn extract_json_handler(item: web::Json<UserInfo>, req: HttpRequest) -> HttpResponse {
    let mut header_map: HashMap<&str, &str> = HashMap::new();
    for (k, v) in req.headers().iter() {
        if let Ok(v1) = v.to_str() {
            header_map.insert(k.as_str(), v1);
        }
    }

    info!("request headers: {:?}", json!(header_map).to_string());
    HttpResponse::Ok().json(item.0) // <- send json response
}

const MAX_SIZE: usize = 262_144; // max payload size is 256k

#[post("/bytes")]
pub async fn bytes_handler(body: web::Bytes) -> Result<HttpResponse, Error> {
    let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
    let res = match result {
        Ok(v) => v,
        Err(e) => json::object! {"err" => e.to_string() },
    };
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(res.dump()))
}

#[post("/payload")]
pub async fn payload_handler(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    let obj = serde_json::from_slice::<UserInfo>(&body)?;
    Ok(HttpResponse::Ok().json(obj)) // <- send response
}
