use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::web::{BytesMut, Payload};
use actix_web::{error, dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage};
use futures::future::{ok, Future, Ready};
use futures::stream::StreamExt;
use actix_web::http::{StatusCode};
use log::Level::Debug;
use crate::error::user_error;

pub struct ReadReqBody;

impl<S: 'static, B> Transform<S, ServiceRequest> for ReadReqBody
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    // type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ReadReqBodyMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ReadReqBodyMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct ReadReqBodyMiddleware<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Service<ServiceRequest> for ReadReqBodyMiddleware<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
        B: 'static,
{
    // type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    // fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
    //     self.service.poll_ready(cx)
    // }
    actix_service::forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        info!("Hi from start. You requested: {}", req.path());
        let mut svc = self.service.clone();
        Box::pin(async move {

            let mut body = BytesMut::new();
            let mut stream = req.take_payload();
            while let Some(chunk) = stream.next().await {
                body.extend_from_slice(&chunk?);
            }
            info!("read body: {:?}", body);
            // 回写body
            let mut payload = actix_http::h1::Payload::empty();
            payload.unread_data(body.into());
            req.set_payload(payload.into());

            let res = svc.call(req).await?;

            info!("response: {:?}", res.headers());
            Ok(res)
            // Err(Error::from(myRespError::MyError::BadClientData))
        })

    }
}
