use std::fmt::Debug;
use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};
use actix_web::{post, web, HttpResponse, get, delete};

use sqlite::Connection;
use std::path::Path;
use actix_web::web::Json;
use chrono::Local;
use validator::{Validate, ValidationError};
use sqlite;
use uuid::Uuid;

macro_rules! ok (($result:expr) => ($result.unwrap()));

fn gen_courses() -> Vec<Course> {
    let connection = setup_users("actix-web-example.db");
    let query = "SELECT * FROM courses";
    let mut statement = ok!(connection.prepare(query));

    let mut courses = Vec::new();
    for row in statement.iter().map(|row| ok!(row)) {
        let fmt = "%Y-%m-%d %H:%M:%S";
        let str_date = row.read::<&str, _>("time");
        let result = NaiveDateTime::parse_from_str(str_date, fmt);
        let date_time = result.unwrap();

        courses.push(Course {
            id: Option::from(row.read::<&str, _>("id").to_string()),
            teacher_id: row.read::<i64, _>("teacher_id"),
            name: Option::from(row.read::<&str, _>("name").to_string()),
            time: Option::from(date_time),
            description: Option::from(row.read::<&str, _>("description").to_string()),
            format: Option::from(row.read::<&str, _>("format").to_string()),
            structure: Option::from(row.read::<&str, _>("structure").to_string()),
            duration: Option::from(row.read::<&str, _>("duration").to_string()),
            price: Option::from(row.read::<f64, _>("price")),
            language: Option::from(row.read::<&str, _>("language").to_string()),
            level: Option::from(row.read::<&str, _>("level").to_string()),
        })
    }

    courses
}


pub fn setup_users<T: AsRef<Path>>(path: T) -> Connection {
    let connection = ok!(sqlite::open(path));
    let query = "
     CREATE TABLE courses (id TEXT, teacher_id INTEGER, name TEXT, time TEXT, description TEXT, format TEXT, structure TEXT, duration TEXT, price DOUBLE, language TEXT, level TEXT);".to_owned();
    connection.execute(query).unwrap_or_else(|err| {
        debug!("{}", err)
    });
    connection
}

#[get("/courses")]
pub async fn get_courses() -> HttpResponse {
    // sqlite_test();
    HttpResponse::Ok()
        // .insert_header(("content-type", "application/json"))
        .json(gen_courses())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HttpError {
    code: String,
    msg: String,
}


#[post("/courses")]
pub async fn add_courses(info: web::Json<Course>) -> HttpResponse {
    match insert_test(info) {
        Err(e) => {
            let msg = HttpError {
                code: "ACTIX_000001".parse().unwrap(),
                msg: e.to_string(),
            };
            return HttpResponse::BadRequest().json(msg);
        }
        _ => {}
    };
    // debug!("add courses: {}", info.name.unwrap());
    HttpResponse::Ok()
        // .insert_header(("content-type", "application/json"))
        .json(gen_courses())
}

#[delete("/courses/{course_id}")]
pub async fn del_courses(course_id: web::Path<String>) -> HttpResponse {
    delete_test(course_id.into_inner());
    HttpResponse::Ok()
        // .insert_header(("content-type", "application/json"))
        .json(gen_courses())
}

fn validate_unique_username(username: &str) -> Result<(), ValidationError> {
    if username == "xXxShad0wxXx" {
        // the value of the username will automatically be added later
        return Err(ValidationError::new("invalid name"));
    }
    Ok(())
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct Course {
    pub id: Option<String>,
    #[validate(range(min = 1, max = 32))]
    pub teacher_id: i64,
    #[validate(custom(function = "validate_unique_username", message = "invalid name"))]
    pub name: Option<String>,
    pub time: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<f64>,
    pub language: Option<String>,
    pub level: Option<String>,
}

fn delete_test(course_id: String) {
    let connection = setup_users("actix-web-example.db");
    let query = "DELETE FROM courses WHERE id=:id;";
    let mut statement = ok!(connection.prepare(query));
    statement.bind((":id", course_id.as_str())).expect("");
    ok!(statement.next());
    ok!(statement.reset());
}

fn insert_test(info: Json<Course>) -> Result<(), &'static str> {
    info.validate().unwrap();
    let connection = setup_users("actix-web-example.db");
    let query = "INSERT INTO courses VALUES (:id, :teacher_id, :name, :time, :description, :format, :structure, :duration, :price, :language, :level)";
    let mut statement = connection.prepare(query).expect("prepare error");
    let fmt = "%Y-%m-%d %H:%M:%S";
    let dft = Local::now().format(fmt);
    let result = NaiveDateTime::parse_from_str(dft.to_string().as_str(), fmt);
    let date_time = result.unwrap();
    let id = Uuid::new_v4();

    statement.bind((":id", id.to_string().as_str())).expect("");
    statement.bind((":teacher_id", info.teacher_id.clone())).expect("");
    statement.bind((":name", info.name.clone().unwrap_or_default().as_str())).expect("");
    statement.bind((":time", date_time.to_string().as_str())).expect("");
    statement.bind((":description", info.description.clone().unwrap_or_default().as_str())).expect("");
    statement.bind((":format", info.format.clone().unwrap_or_default().as_str())).expect("");
    statement.bind((":structure", info.structure.clone().unwrap_or_default().as_str())).expect("");
    statement.bind((":duration", info.duration.clone().unwrap_or_default().as_str())).expect("");
    statement.bind((":price", info.price.clone().unwrap_or_default())).expect("");
    statement.bind((":language", info.language.clone().unwrap_or_default().as_str())).expect("");
    statement.bind((":level", "1")).expect("");
    ok!(statement.next());
    ok!(statement.reset());
    Ok(())
}
