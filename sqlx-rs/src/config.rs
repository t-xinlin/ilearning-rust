use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Database {
    pub name: String,
    pub user: String,
    pub typ: String,
    pub pwd: String,
    pub host: String,
    pub connection_max: toml::Value,
    pub enabled: toml::Value,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub title: String,
    pub database: Database,
}
