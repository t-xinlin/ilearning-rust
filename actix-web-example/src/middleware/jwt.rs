use actix_web::dev::{/*Transform, Service, */ServiceRequest, ServiceResponse};
use futures::future::{Ready, ok};
use futures::{Future};
use futures::stream::StreamExt;
use actix_web::{Error, HttpMessage};
use std::task::{Context, Poll};
use std::pin::Pin;
use std::rc::Rc;
use std::cell::RefCell;
use actix_web::web::BytesMut;
use actix_service::{Service, Transform};
use std::borrow::BorrowMut;


pub struct Jwt;

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
            let mut stream = req.borrow_mut().take_payload();
            while let Some(chunk) = stream.next().await {
                body.extend_from_slice(&chunk?);
            }
            info!("jwt body: {:?}", body);
            let res = svc.call(req).await?;
            info!("jwt response: {:?}", res.headers());
            Ok(res)

            // Err(Error::from(myRespError::MyError::BadClientData))
        })
    }
}
