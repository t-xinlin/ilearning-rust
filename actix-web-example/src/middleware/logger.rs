#![allow(unused_imports)]

use std::pin::Pin;
use std::task::{Context, Poll};
use std::rc::Rc;
use std::cell::RefCell;
use futures::{future::{ok, Future, Ready}, stream::StreamExt};

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
// use actix_web::body::to_bytes;
use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    web::{Bytes, BytesMut},
    web,
    body::{BoxBody, BodySize, MessageBody, to_bytes},
    http::Version,
    Error, HttpMessage,
};
use actix_http::h1::{Payload};

use chrono::{Utc, DateTime, NaiveDate, NaiveDateTime};
use pin_project::__private::PinnedDrop;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Logging;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body


impl<S: 'static, B> Transform<S, ServiceRequest> for Logging
    where S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
          S::Future: 'static,
          B: MessageBody + Unpin + 'static,
{
    // impl<S, B> Transform<S> for Logging
//     where
//         S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
//         S::Future: 'static,
//         B: MessageBody + Unpin + 'static,
// {
    // type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = LoggingMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoggingMiddleware {
            service: Rc::new(RefCell::new(service))
        })
    }
}

pub struct LoggingMiddleware<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddleware<S>
    where S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
          S::Future: 'static,
          B: MessageBody + Unpin + 'static,
{
    // impl<S, B> Service for LoggingMiddleware<S>
//     where
//         S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
//         S::Future: 'static,
//         B: MessageBody + Unpin + 'static,
// {
    // type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    // fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
    //     self.service.poll_ready(cx)
    // }
    actix_service::forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            let begin = Utc::now();
            let path = req.path().to_string();
            let method = req.method().as_str().to_string();
            let queries = req.query_string().to_string();
            let ip = req.head().peer_addr;
            match req.version() {
                Version::HTTP_09 => "http/0.9",
                Version::HTTP_10 => "http/1.0",
                Version::HTTP_11 => "http/1.1",
                Version::HTTP_2 => "http/2.0",
                Version::HTTP_3 => "http/3.0",
                _ => "UNKNOWN",
            };

            // Request headers
            let mut domain = String::new();
            let mut user_agent = String::new();
            let mut headers: HashMap<&str, &str> = HashMap::new();
            for (k, v) in req.headers().iter() {
                if let Ok(inner) = v.to_str() {
                    let key = k.as_str();
                    headers.insert(key, inner);
                    match key {
                        "host" => { domain = inner.to_string() }
                        "user-agent" => { user_agent = inner.to_string() }
                        _ => {}
                    }
                }
            }
            let req_headers = json!(headers).to_string();

            let req_body = get_request_body(&mut req).await;
            let parsed = parse_body(req_body.unwrap());
            let req_body: Option<String> = if !parsed.is_object() {
                None
            } else {
                /*
                // my code
                // Mask some words for security, like `password`
                for k in vec!["password"] {
                    let obj = parsed.as_object_mut().unwrap();
                    if let Some(p) = obj.get_mut(k) {
                        *p = json!("MASKED_FOR_SECURITY");
                    }
                }
                */
                Some(parsed.to_string())
            };

            // DbPool
            /*
            // my code
            let pool = req.app_data::<web::Data<DbPool>>().map(|p| p.clone());
            */

            // Exec main function and wait response generated
            let res = svc.call(req).await?;

            let duration = (Utc::now() - begin).num_microseconds();
            let status_code = res.status();

            // Response headers
            let mut headers: HashMap<&str, &str> = HashMap::new();
            for (k, v) in res.headers().iter() {
                if let Ok(inner) = v.to_str() {
                    headers.insert(k.as_str(), inner);
                }
            }
            let res_headers = json!(headers).to_string();

            // Get response body
            let res_body = BytesMut::new();
            // let mut stream = res.into_body();
            // let body_bytes = to_bytes(res.into_body()).await?;
            // let body_bytes = match to_bytes(res.into_body()).await {
            //     Ok(b) => {
            //         while let Some(chunk) = b.into_iter().next().await {
            //             res_body.extend_from_slice(&chunk?);
            //         }
            //     }
            //     _ => {}
            // };
            // // let mut stream = res.take_body();
            // while let Some(chunk) = body_bytes.next().await {
            //     res_body.extend_from_slice(&chunk?);
            // }

            // Logging
            println!("req.domain  : {:?}", domain);
            println!("req.user_agent  : {:?}", user_agent);
            println!("req.ip  : {:?}", ip);
            println!("req.path  : {:?}", path);
            println!("req.method : {:?}", method);
            println!("req.headers: {:?}", req_headers);
            println!("req.query : {:?}", queries);
            println!("req.body  : {:?}", req_body);
            println!("duration : {:?}", duration);
            println!("res.status : {:?}", status_code);
            println!("res.headers: {:?}", res_headers);
            println!("res.body  : {:?}", res_body);

            /*
            // my code
            let a = AccessLog {
                id: None,
                protocol: Some(protocol.to_string()).into_iter().find(|v| v != ""),
                domain: Some(domain).into_iter().find(|v| v != ""),
                ip: ip.map(|inner| inner.to_string()).into_iter().find(|v| v != ""),
                method: Some(method).into_iter().find(|v| v != ""),
                path: Some(path.to_string()).into_iter().find(|v| v != ""),
                query: Some(queries).into_iter().find(|v| v != ""),
                user_agent: Some(user_agent),
                req_headers: Some(req_headers).into_iter().find(|v| v != ""),
                req_body: req_body,
                duration: duration.map(|inner| inner as i32),
                status_code: Some(status_code.as_u16() as i32),
                res_headers: Some(res_headers).into_iter().find(|v| v != ""),
                res_body: String::from_utf8(res_body.clone().to_vec()).into_iter().find(|v| v != ""),
                others: None,
                requested_at: Some(begin.with_timezone(&*TIMEZONE).naive_local()),
                created_at: None,
            };

            if let Some(pool) = pool {
                if let Ok(conn) = pool.get() {
                    if let Err(e) = a.create(&conn) {
                        eprintln!("database err: {:?}", e);
                    };
                }
            }
            */

            // return original response body
            // Ok(res.map_body(|_, _b| ResponseBody::Other(Body::from(res_body))))
            // Ok("ok".to_string())
            Ok(res)
        })
    }
}

#[pin_project::pin_project(PinnedDrop)]
pub struct BodyLogger<B> {
    #[pin]
    body: B,
    body_accum: BytesMut,
}

#[pin_project::pinned_drop]
impl<B> PinnedDrop for BodyLogger<B> {
    fn drop(self: Pin<&mut Self>) {
        println!("response body: {:?}", self.body_accum);
    }
}

impl<B: MessageBody> MessageBody for BodyLogger<B> {
    type Error = B::Error;

    fn size(&self) -> BodySize {
        self.body.size()
    }

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        let this = self.project();
        match this.body.poll_next(cx) {
            Poll::Ready(Some(Ok(chunk))) => {
                this.body_accum.extend_from_slice(&chunk);
                Poll::Ready(Some(Ok(chunk)))
            }
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

async fn get_request_body(req: &mut ServiceRequest) -> Result<BytesMut, Error> {
    // Get body as bytes
    let mut bytes: BytesMut = BytesMut::new();
    let mut body = req.take_payload();
    while let Some(chunk) = body.next().await {
        bytes.extend_from_slice(&chunk?);
    }

    // Set body again
    let (_, mut payload) = Payload::create(true);
    payload.unread_data(web::Bytes::from(bytes.clone()));
    req.set_payload(payload.into());

    Ok(bytes)
}

#[derive(Debug, Deserialize, Serialize)]
struct Password {
    password: String,
}

fn parse_body(body: BytesMut) -> Value {
    let json_parsed = serde_json::from_slice::<Value>(&body);
    if let Ok(b) = json_parsed {
        return b;
    }
    // let query_parsed = serde_qs::from_bytes::<Password>(&body);
    json!(null)
}
