use crate::conf::config;
use crate::model;

use std::fmt::Debug;

use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use actix_web::{post, web, HttpResponse, get, delete};
use actix_web::web::Json;
use sqlx;
use futures::TryStreamExt;
use sqlx::Row;
use uuid::Uuid;
use validator::Validate;

// macro_rules! ok (($result:expr) => ($result.unwrap()));

async fn gen_courses() -> Result<Vec<model::Course>, &'static str> {
    let conn = config::SQLITE_CONN.clone();
    let query = "SELECT * FROM courses";
    let mut rows = sqlx::query(query)
        .fetch(&conn);

    let mut courses = Vec::new();
    while let Some(row) = rows.try_next().await.unwrap() {
        let fmt = "%Y-%m-%d %H:%M:%S";
        let str_date = row.try_get("time").unwrap();
        let result = NaiveDateTime::parse_from_str(str_date, fmt);
        let date_time = result.unwrap();

        let id: String = row.try_get("id").unwrap();
        // let teacher_id =  row.try_get("teacher_id").unwrap();
        let name: String = row.try_get("name").unwrap();
        let description: String = row.try_get("description").unwrap();
        let format: String = row.try_get("format").unwrap();
        let structure: String = row.try_get("structure").unwrap();
        let duration: String = row.try_get("duration").unwrap();
        let price: f64 = row.try_get("price").unwrap();
        let language: String = row.try_get("language").unwrap();
        let level: String = row.try_get("level").unwrap();

        courses.push(model::Course {
            id: Option::from(id),
            teacher_id: row.try_get("email").unwrap_or_default(),
            name: Option::from(name),
            time: Option::from(date_time),
            description: Option::from(description),
            format: Option::from(format),
            structure: Option::from(structure),
            duration: Option::from(duration),
            price: Option::from(price),
            language: Option::from(language),
            level: Option::from(level),
        })
    }
    Ok(courses)
}

#[get("/courses")]
pub async fn get_courses() -> HttpResponse {
    let r = gen_courses().await.unwrap();
    HttpResponse::Ok().json(r)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HttpError {
    code: String,
    msg: String,
}

#[post("/courses")]
pub async fn add_courses(info: web::Json<model::Course>) -> HttpResponse {
    let result = insert_test(info).await;
    match result {
        Err(e) => {
            let msg = HttpError {
                code: "ACTIX_000001".parse().unwrap(),
                msg: e.to_string(),
            };
            return HttpResponse::BadRequest().json(msg);
        }
        _ => {}
    };

    let r = gen_courses().await.unwrap();
    HttpResponse::Ok().json(r)
}

#[delete("/courses/{course_id}")]
pub async fn del_courses(course_id: web::Path<String>) -> HttpResponse {
    delete_test(course_id.into_inner()).await.unwrap();
    let r = gen_courses().await.unwrap();
    HttpResponse::Ok().json(r)
}

async fn delete_test(course_id: String) -> Result<(), &'static str> {
    // let connection = config::CONN.lock().unwrap();  // setup_users("actix-web-example.db");
    // let query = "DELETE FROM courses WHERE id=:id;";
    // let mut statement = ok!(connection.prepare(query));
    // statement.bind((":id", course_id.as_str())).expect("");
    // ok!(statement.next());
    // ok!(statement.reset());
    let conn = config::SQLITE_CONN.clone();
    let query = "DELETE FROM courses WHERE id=?";
    let _last_id = sqlx::query(query)
        .bind(course_id)
        .execute(&conn)
        .await.unwrap()
        .last_insert_rowid();

    Ok(())
}

async fn insert_test(info: Json<model::Course>) -> Result<(), &'static str> {
    info.validate().unwrap();
    let conn = config::SQLITE_CONN.clone();
    let query = "INSERT INTO courses VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";

    // let mut statement = connection.prepare(query).expect("prepare error");
    let fmt = "%Y-%m-%d %H:%M:%S";
    let dft = Local::now().format(fmt);
    let result = NaiveDateTime::parse_from_str(dft.to_string().as_str(), fmt);
    let date_time = result.unwrap();
    let id = Uuid::new_v4();

    let _last_id = sqlx::query(query)
        .bind(id.to_string().as_str())
        .bind(info.teacher_id.clone())
        .bind(info.name.clone().unwrap_or_default().as_str())
        .bind(date_time.to_string().as_str())
        .bind(info.description.clone().unwrap_or_default().as_str())
        .bind(info.format.clone().unwrap_or_default().as_str())
        .bind(info.structure.clone().unwrap_or_default().as_str())
        .bind(info.duration.clone().unwrap_or_default().as_str())
        .bind(info.price.clone().unwrap_or_default())
        .bind(info.language.clone().unwrap_or_default().as_str())
        .bind("1")
        .execute(&conn)
        .await.unwrap()
        .last_insert_rowid();
    Ok(())
}
