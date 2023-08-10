use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
// use lazy_static::lazy::Lazy;

use serde::Deserialize;
use toml;
use validator::{Validate, ValidationError};
use once_cell::sync::Lazy;
// use sqlite::Connection;
use sqlx::{Connection, Executor, Pool, Sqlite, sqlite::SqlitePoolOptions, SqliteConnection};
use sqlx::migrate::MigrateDatabase;

lazy_static::lazy_static! {
    pub static ref GLOBAL_CONFIG: Arc<Mutex<Conf>> = Arc::new(Mutex::new(Conf::new().unwrap()));
    // pub static ref CONN: Arc<Mutex<Pool<Sqlite>>> = Arc::new(Mutex::new(setup_db("actix-web-example.db").unwrap()));
    // pub static ref SQLITE_CONN: Arc<Mutex<Pool<Sqlite>>> = Arc::new(Mutex::new(setup_db("actix-web-example.db").unwrap()));
}

pub static SQLITE_CONN: Lazy<Pool<Sqlite>> = Lazy::new(|| setup_db("actix-web-example.db").unwrap());

// lazy_static::lazy_static! {
//     pub static ref CONN: Arc<Mutex<Connection>> = Arc::new(Mutex::new(setup_users("actix-web-example.db")));
// }
// lazy_static::lazy_static! {
//     pub static ref CACHE:Arc<Mutex<Connection>> = Arc::new(Mutex::new(Conf::new().unwrap()));
// }


pub async fn init_db() {
    Sqlite::create_database("actix-web-example.db").await.expect("init db error");
    let mut conn = SqliteConnection::connect("actix-web-example.db").await.unwrap();
    let query = "CREATE TABLE IF NOT EXISTS courses (\
            id TEXT, \
            teacher_id INTEGER, \
            name TEXT, \
            time TEXT, \
            description TEXT, \
            format TEXT, \
            structure TEXT, \
            duration TEXT, \
            price DOUBLE, \
            language TEXT, \
            level TEXT);";
    conn.execute(query).await.unwrap();
}

pub fn setup_db<T: AsRef<Path>>(_path: T) -> Result<Pool<Sqlite>, sqlx::Error> {
    debug!("init db");
    let pool = SqlitePoolOptions::new()
        .max_connections(100)
        .min_connections(10)
        // .connect_timeout(std::time::Duration::from_secs(10))
        .max_lifetime(std::time::Duration::from_secs(1800))
        .idle_timeout(std::time::Duration::from_secs(600))
        // .connect("actix-web-example.db"). await?.unwrap();
        .connect_lazy("actix-web-example.db").unwrap();
    Ok(pool)
}

#[derive(Deserialize, Debug, Validate, Clone)]
pub struct Package {
    pub name: Option<String>,
    pub version: Option<String>,
    pub authors: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Validate, Clone)]
pub struct Server {
    pub name: Option<String>,
    pub address: Option<String>,
    pub port: Option<i64>,
    pub services: Option<Vec<Address>>,
}

#[derive(Deserialize, Debug, Validate, Clone)]
pub struct Address {
    pub address: Option<String>,
    #[validate(custom(function = "validate_port", message = "invalid port"))]
    pub port: Option<i64>,
}

#[derive(Deserialize, Debug, Validate, Clone)]
pub struct Log {
    file: Option<String>,
}

#[derive(Deserialize, Debug, Validate, Clone)]
pub struct Database {
    pub db_type: Option<String>,
    pub host: Option<String>,
    pub port: Option<i64>,
    pub name: Option<String>,
    pub user: Option<String>,
    pub password: Option<String>,
    pub ssl_enable: Option<bool>,
    pub max_idle: Option<String>,
    pub max_open: Option<String>,
}

#[derive(Deserialize, Debug, Validate)]
pub struct Conf {
    #[validate]
    pub package: Package,
    #[validate]
    pub log: Log,
    #[validate]
    pub server: Server,

}

fn validate_port(p: i64) -> Result<(), ValidationError> {
    if p == 0 {
        // the value of the username will automatically be added later
        // return Err(ValidationError::new("invalid name"));
        debug!("invalid name")
    }
    Ok(())
}

impl Conf {
    pub fn new() -> Result<Conf, &'static str> {
        let mut cwd = env::current_dir().unwrap();
        cwd.push(Path::new("conf"));
        cwd.push(Path::new("app.toml"));
        let conf_path = cwd.to_str().unwrap();
        let mut file = match File::open(conf_path) {
            Ok(f) => f,
            Err(e) => panic!("open file {} exception:{}", conf_path, e)
        };
        let mut str_val = String::new();
        match file.read_to_string(&mut str_val) {
            Ok(s) => s,
            Err(e) => panic!("Error Reading file: {}", e)
        };
        let config: Conf = toml::from_str(&str_val).unwrap();
        // validate config
        config.validate().unwrap();
        let s = config.clone();
        debug!("config.package.name:{:?}", s.package.name);
        debug!("config.package.version:{:?}", s.package.version);
        debug!("config.package.authors:{:?}", s.package.authors);
        debug!("config.log.file:{:?}", s.log.file);
        Ok(config)
    }
}

impl Clone for Conf {
    fn clone(&self) -> Self {
        Conf {
            package: self.package.clone(),
            log: self.log.clone(),
            server: self.server.clone(),
        }
    }
}
