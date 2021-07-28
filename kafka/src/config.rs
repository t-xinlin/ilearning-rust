use log::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    global_string: Option<String>,
    global_integer: Option<u64>,
    server: Option<ServerConfig>,
    peers: Option<Vec<PeerConfig>>,
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    ip: Option<String>,
    port: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct KafkaConfig {
    pub topics: Option<String>,
    pub config: HashMap<String, Option<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Kafka {
    pub topics: Option<Vec<String>>,
    pub topic: Option<String>,
    pub config: HashMap<String, Option<String>>,
    // pub producer: HashMap<String, Option<String>>,
}

#[derive(Debug, Deserialize)]
struct PeerConfig {
    ip: Option<String>,
    port: Option<u64>,
}

pub fn init_config() {
    let toml_str = r#"
        global_string = "test"
        global_integer = 5
        [server]
        ip = "127.0.0.1"
        port = 80
        [[peers]]
        ip = "127.0.0.1"
        port = 8080
        [[peers]]
        ip = "127.0.0.1"
    "#;
    let decoded: Config = toml::from_str(toml_str).unwrap();
    info!("{:#?}", decoded);
}

pub fn init_kafka() -> Kafka {
    let mut cwd = env::current_dir().unwrap();
    cwd.push(Path::new("conf"));
    cwd.push(Path::new("config.toml"));
    info!("init log path:{}", cwd.to_str().unwrap());
    let conf_path = cwd.to_str().unwrap();

    let mut file = std::fs::File::open(conf_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    info!("read file: {:?}", contents);
    let decoded: Kafka = toml::from_str(contents.as_str()).unwrap();
    info!("decoded: {:#?}", decoded);
    // Kafka {
    //     group_id: None,
    //     topics: None,
    //     brokers: None,
    //     offset_reset: None,
    //     kerberos_service_name: None,
    //     kerberos_keytab: None,
    //     KerberosPrincipal: None,
    //     Protocol: None,
    // }
    decoded
}
