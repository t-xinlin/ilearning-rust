use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;

use serde::Deserialize;
use toml;

use validator::{Validate, ValidationError};

lazy_static::lazy_static! {
    pub static ref CACHE:  Arc<Mutex<Conf>> =    Arc::new(Mutex::new(Conf::new().unwrap()));
}

fn validate_username(username: &str) -> Result<(), ValidationError> {
    if username == "xXxShad0wxXx" {
        // the value of the username will automatically be added later
        // return Err(ValidationError::new("invalid name"));
        debug!("invalid name")
    }
    Ok(())
}

#[derive(Deserialize, Debug, Validate, Clone)]
pub struct Server {
    address: Option<String>,
    port: Option<i64>,
}

#[derive(Deserialize, Debug, Validate, Clone)]
pub struct Log {
    #[validate(custom(function = "validate_username", message = "invalid name"))]
    username: String,
    file: Option<String>,
}

#[derive(Deserialize, Debug, Validate, Clone)]
pub struct Conf {
    #[validate]
    pub server: Server,
    #[validate]
    pub log: Log,
    // #[validate]
    // pub ip_config: Option<Vec<IpConfig>>,
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
        debug!("config.server.address:{:?}", s.server.address);
        debug!("config.server.port:{:?}", s.server.port);
        debug!("config.log.username:{:?}", s.log.username);
        debug!("config.log.file:{:?}", s.log.file);
        Ok(config)
    }
}
