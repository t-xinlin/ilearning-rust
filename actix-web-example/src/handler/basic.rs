use actix_web::{
    get,
    web,
    Responder,
    HttpRequest,
};
use std::sync::Mutex;
// use actix_http::http::ContentEncoding;

#[get("/index.html")]
pub async fn index(req: HttpRequest) -> impl Responder {
    format!("Hello {} {}!", req.path().to_string(), "index2")
}

// // #[get("/test/{id}/{name}")]
// async fn test(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
//     println!("{}{}", id, name);
//     format!("Hello {}! id:{}", name, id)
// }


pub async fn greet(req: HttpRequest) -> impl Responder {
    println!("greet req");
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

pub struct AppStateWithCounter {
    pub counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

pub fn new_counter() -> web::Data<AppStateWithCounter> {
    web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    })
}

pub async fn state(data: web::Data<AppStateWithCounter>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard
    format!("Request number: {}", counter) // <- response with count
}
