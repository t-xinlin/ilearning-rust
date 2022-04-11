use std::collections::HashMap;
use reqwest::header::HeaderMap;
use serde_json::value::Value;
use std::path::{Path};
use std::{env, time};
// use std::str::FromStr;
use log::*;
use serde::{Deserialize, Serialize};
use tokio_cron_scheduler::{JobScheduler, Job};

fn init_log() {
    let mut cwd = env::current_dir().unwrap();
    let p = Path::new("conf/log4rs.yaml");
    cwd.push(p);
    println!("log config file: {}", cwd.to_str().unwrap());
    let conf_path = cwd.to_str().unwrap();
    log4rs::init_file(conf_path, Default::default()).unwrap();
}

#[tokio::main]
async fn main() {
    init_log();
    let expression: &str = "1/1 * * * * *";
    let mut sched = JobScheduler::new();
    // sched.shutdown_on_ctrl_c();
    sched.set_shutdown_handler(Box::new(|| {
        Box::pin(async move {
            println!("Shut down done");
        })
    })).unwrap();

    let four_s_job_async = Job::new_async(expression, |_uuid, _l| {
        Box::pin(async move {
            let res = get_list_req("http://127.0.0.1:8080/app/bytes").await;
            info!("bytes body: {:?}", res);
            let res = get_list_req("http://127.0.0.1:8080/app/json").await;
            info!("json body: {:?}", res);
            let res = get_list_req("http://127.0.0.1:8080/app/extract_json").await;
            info!("extract_json body: {:?}", res);
            let res = get_list_req("http://127.0.0.1:8080/app/payload").await;
            info!("payload body: {:?}", res);
            // match res {
            //     Ok(o) => {
            //         if let Some(code) = o.code {
            //             info!("code ok {:?}",code)
            //         } else {
            //             info!("code err {:?}", o)
            //         }
            //     }
            //     Err(e) => {
            //         error!("{:?}", e)
            //     }
            // }
        })
    }).unwrap();
    sched.add(four_s_job_async).unwrap();
    sched.start().await.unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseBody {
    code: Option<i32>,
    data: Option<HashMap<String, Value>>,
    msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseData {
    code: Option<i32>,
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: i32,
}

async fn get_list_req(host_path : &str) -> Result<ResponseBody, reqwest::Error> {
    let client = reqwest::Client::builder().no_proxy().build().expect("should be able to build reqwest client");
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("test_header", "a9999".parse().unwrap());
    headers.insert("test_header1", "a9999".parse().unwrap());
    headers.insert("sign", "123456".parse().unwrap());
    let new_post = ResponseBody {
        code: Some(1),
        data:None,
        msg: "req msg".to_string(),
    };
    let body = client.post(host_path)
        .headers(headers)
        .timeout(time::Duration::from_secs(10))
        .json(&new_post)
        .send()
        .await?
        .json::<ResponseBody>()
        .await?;
    Ok(body)
}

//
//
// // get请求不含token，输出json 格式
// async fn get_calss_list_out_json()->Result<(),Box<dyn std::error::Error>>{
//     let res=reqwest::get("http://example.url")
//         .await?
//         .json::<HashMap<String,Value>>()
//         .await?;
//     println!("{:#?}",res);
//     Ok(())
// }
//
// // 输出 text 格式
// async fn get_calss_list_out_text()->Result<(),reqwest::Error>{
//     let res=reqwest::get("http://example.url")
//         .await?
//         .text()
//         .await?;
//     Ok(println!("{:#?}",res))
//
// }
//
// async fn get_column_list()->Result<HashMap<String, Value>, reqwest::Error>{
//     // post 请求要创建client
//     let client = reqwest::Client::new();
//
//     // 组装header
//     let mut headers = HeaderMap::new();
//     headers.insert("Content-Type", "application/json".parse().unwrap());
//     headers.insert("Authorization", "Bearer  token_in_here".parse().unwrap());
//     // post 参数
//     // 组装要提交的数据
//     let mut data = HashMap::new();
//     data.insert("params", "1");
//     Ok(client.post("https://http://example.url").headers(headers).json(&data).send().await?.json::<HashMap<String, Value>>().await?)
// }
//
// async fn get_other() -> Result<(),reqwest::Error> {
//     let mut data = HashMap::new();
//     data.insert("params1", "1");
//     data.insert("params2", "2");
//     data.insert("params3", "{\"channel_name\":\"新闻广播\",\"logo\":\"\",\"desc\":\"123\"}");
//     data.insert("compereName", "主持人");
//     data.insert("status", "1");
//
//     let res = reqwest::Client::new()
//         .post("http://example.url")
//         .json(&data)
//         .header("Content-Type", "application/x-www-form-urlencoded")
//         .header("Access-Control-Allow-Origin", "*")
//         .send()
//         .await?;
//     let text = res.text().await?;
//     Ok(println!("{:#?}",text))
//
// }
