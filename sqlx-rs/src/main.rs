mod config;

use serde::{Deserialize, Serialize};
use sqlx::mysql::{MySql, MySqlConnection, MySqlPool, MySqlPoolOptions, MySqlRow};
use std::env;
use structopt::StructOpt;
// provides `try_next`
use anyhow::{Error, Result};
use futures::TryStream;
use futures::TryStreamExt;
use sqlx::types::chrono;
use sqlx::{Column, Connection, Executor, FromRow, Pool, Row, Statement, TypeInfo};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct BizActivity {
    pub id: String,
    pub name: String,
    pub pc_link: Option<String>,
    pub h5_link: Option<String>,
    pub sort: String,
    pub status: i32,
    pub version: i32,
    pub remark: Option<String>,
    pub create_time: chrono::DateTime<chrono::Utc>,
    pub delete_flag: i32,
    pub pc_banner_img: Option<String>,
    pub h5_banner_img: Option<String>,
}

impl BizActivity {
    pub fn new() -> BizActivity {
        BizActivity {
            id: "".to_string(),
            name: "".to_string(),
            pc_link: None,
            h5_link: None,
            sort: "".to_string(),
            status: 0,
            version: 0,
            remark: None,
            create_time: chrono::Utc::now(),
            delete_flag: 0,
            pc_banner_img: None,
            h5_banner_img: None,
        }
    }
}

pub async fn find_all(pool: &Pool<MySql>) -> Result<Vec<BizActivity>> {
    let mut todos = vec![];
    let recs = sqlx::query_as::<_, BizActivity>("SELECT * FROM biz_activity")
        .fetch_all(pool)
        .await?;

    for rec in recs {
        todos.push(BizActivity {
            id: rec.id,
            name: rec.name,
            pc_link: rec.pc_link,
            h5_link: rec.h5_link,
            sort: rec.sort,
            status: rec.status,
            version: rec.version,
            remark: rec.remark,
            create_time: rec.create_time,
            delete_flag: rec.delete_flag,
            pc_banner_img: rec.pc_banner_img,
            h5_banner_img: rec.h5_banner_img,
        });
    }
    Ok(todos)
}

pub async fn create(id: i32, biz: BizActivity, pool: &Pool<MySql>) -> Result<u64> {
    let mut tx = pool.begin().await?;
    let todo =
        sqlx::query("INSERT INTO biz_activity (id, name, sort, status, version, create_time, delete_flag) VALUES (?, ?, ?, ?, ?, ?, ?)")
            .bind(id)
            .bind(biz.name)
            .bind(biz.sort)
            .bind(biz.status)
            .bind(biz.version)
            .bind(chrono::Utc::now())
            .bind(biz.delete_flag)
            .execute(&mut tx)
            .await?;

    tx.commit().await?;
    Ok(0)
}

pub async fn delete(id: i32, pool: &Pool<MySql>) -> Result<u64> {
    let mut tx = pool.begin().await?;
    let deleted = sqlx::query("DELETE FROM biz_activity WHERE id = ?")
        .bind(id)
        .execute(&mut tx)
        .await?;
    tx.commit().await?;
    Ok(0)
}

pub async fn update(id: i32, biz: BizActivity, pool: &Pool<MySql>) -> Result<u64> {
    let mut tx = pool.begin().await.unwrap();
    let todo = sqlx::query("UPDATE biz_activity SET name = ?, status = ? WHERE id = ?")
        .bind(biz.name)
        .bind(biz.status)
        .bind(id)
        .execute(&mut tx)
        .await?;

    tx.commit().await.unwrap();
    Ok(0)
}

fn extension_explicit(file_name: &str) -> Option<&str> {
    match find(file_name, '.') {
        None => None,
        Some(i) => Some(&file_name[i + 1..]),
    }
}

#[async_std::main]
// or #[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let mut cwd = env::current_dir().unwrap();
    let confPath = Path::new("conf").join(Path::new("conf.toml"));
    cwd = cwd.join(confPath);
    let confPath = cwd.to_str().unwrap();

    // let file_path = "sample.toml";
    let mut file = match File::open(confPath) {
        Ok(f) => f,
        Err(e) => panic!("no such file {} exception:{}", confPath, e),
    };
    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
        Ok(s) => s,
        Err(e) => panic!("Error Reading file: {}", e),
    };
    let config: config::Config = toml::from_str(&str_val).unwrap();
    println!("======config: {:?}", config);

    let dbStr = format!(
        "{}://{}:{}@{}/{}",
        config.database.typ,
        config.database.user,
        config.database.pwd,
        config.database.host,
        config.database.name
    );
    let pool = MySqlPoolOptions::new()
        .max_connections(100)
        .min_connections(10)
        .connect_timeout(std::time::Duration::from_secs(10))
        .max_lifetime(std::time::Duration::from_secs(1800))
        .idle_timeout(std::time::Duration::from_secs(600))
        .connect(dbStr.as_str())
        .await?;

    let mut newVo = BizActivity::new();
    newVo.status = 1;
    newVo.name = "new name".to_string();
    newVo.version = 1;
    newVo.delete_flag = 0;

    match create(123, newVo, &pool).await {
        Ok(t) => {
            println!("createOK:{:?}", t);
        }
        Err(e) => {
            println!("create error:{:?}", e);
        }
    };

    let mut vo = BizActivity::new();
    vo.status = 1;
    vo.name = "new name for update".to_string();
    match update(123, vo, &pool).await {
        Ok(t) => {
            println!("update OK:{:?}", t);
        }
        Err(e) => {
            println!("update error:{:?}", e);
        }
    };

    match find_all(&pool).await {
        Ok(t) => {
            println!("find_all OK:{:?}", t);
        }
        Err(e) => {
            println!("find_all error:{:?}", e);
        }
    };
    Ok(())
}
