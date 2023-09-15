use std::collections::HashMap;
use std::sync::Mutex;

use actix_web::{
    get,
    web,
    Responder,
    HttpRequest,
    Error,
};

// use actix_http::http::ContentEncoding;
use actix_web_lab::respond::Html;
use askama::Template;

#[derive(Template)]
#[template(path = "user.html")]
struct UserTemplate<'a> {
    name: &'a str,
    text: &'a str,
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

#[get("/index.html")]
async fn index(query: web::Query<HashMap<String, String>>) -> Result<impl Responder, Error> {
    let html = if let Some(name) = query.get("name") {
        UserTemplate {
            name,
            text: "Welcome!",
        }
            .render()
            .expect("template should be valid")
    } else {
        Index.render().expect("template should be valid")
    };

    Ok(Html(html))
}

// #[get("/index.html")]
// pub async fn index2(req: HttpRequest) -> impl Responder {
//     format!("Hello {} {}!", req.path().to_string(), "index2")
// }

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
