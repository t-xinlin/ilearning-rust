use actix_web::dev::{
    ServiceRequest,
    ServiceResponse,
};
use futures::{
    future::{Ready, ok},
    Future,
    stream::StreamExt,
};

use actix_web::{
    ResponseError,
    Error,
    HttpMessage,
    HttpResponse,
    web::{BytesMut, BufMut},
};
use std::pin::Pin;
use std::rc::Rc;
use std::cell::RefCell;

use actix_service::{Service, Transform};
use actix_http::{
    header,
    StatusCode,
    Response,
    body::BoxBody,
};

use std::{io::Write as _};

pub struct Jwt;

#[derive(Debug, derive_more::Display, derive_more::Error)]
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

    fn error_response(&self) -> HttpResponse {
        let mut res = Response::new(self.status_code());
        let mut buf = BytesMut::new().writer();
        let _ = write!(buf, "{}", Error::from(UserError::ValidationError { field: "token invalid".to_string() }).to_string());

        res.headers_mut().insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        HttpResponse::from(res.set_body(BoxBody::new(buf.into_inner())))
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

    actix_service::forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        debug!("call in coming");
        let svc = self.service.clone();
        Box::pin(async move {
            let mut body = BytesMut::new();
            let mut stream = req.take_payload();
            while let Some(chunk) = stream.next().await {
                body.extend_from_slice(&chunk?);
            }
            if let Some(sign) = get_content_type(&req, "sign".to_string()) {
                let v: Vec<&str> = sign.split('.').collect();
                for s in v {
                    debug!("{}", s);
                }
                debug!("get sign = '{}'", sign);
            } else {
                error!("token invalid");
                return Err(Error::from(UserError::ValidationError { field: "token invalid".to_string() }));
            }

            debug!("request body: {:?}", body);
            // 回写body
            let (_, mut payload) = actix_http::h1::Payload::create(true);
            // let mut payload = actix_http::h1::Payload::empty();
            payload.unread_data(body.into());
            req.set_payload(payload.into());

            let res = svc.call(req).await?;
            debug!("response: {:?}", res.headers());
            Ok(res)
            // Err(Error::from(myRespError::MyError::BadClientData))
        })
    }
}

fn get_content_type(req: &ServiceRequest, key: String) -> Option<&str> {
    req.headers().get(key)?.to_str().ok()
}
