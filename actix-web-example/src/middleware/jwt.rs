use actix_web::dev::{/*Transform, Service, */ServiceRequest, ServiceResponse};
use futures::future::{Ready, ok};
use futures::{Future};
use futures::stream::StreamExt;
use actix_web::{ResponseError, Error, HttpMessage, HttpRequest, HttpResponse, HttpResponseBuilder};
use std::task::{Context, Poll};
use std::pin::Pin;
use std::rc::Rc;
use std::cell::RefCell;
use actix_web::web::{BytesMut, Buf, BufMut};
use actix_service::{Service, Transform};
use std::borrow::BorrowMut;
use actix_web::error::PayloadError;

use actix_http::http::{header, HeaderMap, HeaderValue, StatusCode};
use log4rs::init_file;
use actix_http::Protocol::Http1;
// use actix_http::error as actixHttpError;
use derive_more::{Display, Error};
use actix_http::{error, body::Body, Response};
use std::{fmt, io::Write as _};

pub struct Jwt;

#[derive(Debug, Display, Error)]
pub enum UserError {
    #[display(fmt = "Validation error on field: {}", field)]
    ValidationError { field: String },
}

impl ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::ValidationError { .. } => StatusCode::UNAUTHORIZED,
        }
    }
    fn error_response(&self) -> Response<Body> {
        // let mut res = Response::new(self.status_code());
        // res.headers_mut().insert(
        //     header::CONTENT_TYPE,
        //     header::HeaderValue::from_static("application/json"),
        // );
        // res.set_body(Body::from(self.to_string())).into()


        let mut res = Response::new(self.status_code());
        let mut buf = BytesMut::new().writer();
        let _ = write!(buf, "{}", self);

        res.headers_mut().insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        res.set_body(Body::from(buf.into_inner())).into()
    }
}

impl<S: 'static, B> Transform<S, ServiceRequest> for Jwt
    where S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
          S::Future: 'static,
          B: 'static,
{
    // type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        debug!("new_transform in coming");
        ok(JwtMiddleware {
            service: Rc::new(RefCell::new(service))
        })
    }
}

pub struct JwtMiddleware<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Service<ServiceRequest> for JwtMiddleware<S>
    where S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
          S::Future: 'static,
          B: 'static,
{
    // type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    // fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    //     self.service.poll_ready(ctx)
    // }
    actix_service::forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        debug!("call in coming");
        let mut svc = self.service.clone();
        Box::pin(async move {
            let mut body = BytesMut::new();
            let mut stream = req.take_payload();
            while let Some(chunk) = stream.next().await {
                body.extend_from_slice(&chunk?);
            }

            if let Some(sign) = get_content_type(&req, "sign".to_string()) {
                let v: Vec<&str> = sign.split('.').collect();
                for s in v {
                    info!("{}", s);
                }
                info!("get sign = '{}'", sign);
            } else {
                error!("token invalid");
                return Err(Error::from(UserError::ValidationError { field: "token invalid".to_string() }));
            }

            info!("request body: {:?}", body);
            // 回写body
            let mut payload = actix_http::h1::Payload::empty();
            payload.unread_data(body.into());
            req.set_payload(payload.into());

            let res = svc.call(req).await?;
            info!("response: {:?}", res.headers());
            Ok(res)
            // Err(Error::from(myRespError::MyError::BadClientData))
        })

        //
        // let mut svc = self.service.clone();
        // Box::Pin(
        //    async move {
        //        req.take_payload()
        //            .fold(BytesMut::new(), move |mut body, chunk| {
        //                let bs = chunk.unwrap().as_bytes();
        //                body.extend_from_slice(bs);
        //                // Ok::<_, PayloadError>(body)
        //                Ok(())
        //            })
        //            .map_err(|e| Err((e.into())))
        //            .and_then(move |bytes| {
        //                println!("request body: {:?}", bytes);
        //
        //                let mut payload = actix_http::h1::Payload::empty();
        //                payload.unread_data(bytes.into());
        //                req.set_payload(payload.into());
        //
        //                svc.call(req).and_then(|res| Ok(res))
        //            });
        //        Ok(())
        //    }
        // )
    }
}

fn get_content_type(req: &ServiceRequest, key: String) -> Option<&str> {
    req.headers().get(key)?.to_str().ok()
}
