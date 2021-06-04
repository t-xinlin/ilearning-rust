use actix_web::{error as httpError, get, http::header, http::StatusCode, App, HttpResponse, HttpServer, HttpResponseBuilder};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum UserError {
    #[display(fmt = "Validation error on field: {}", field)]
    ValidationError { field: String },
}

impl httpError::ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::ValidationError { .. } => StatusCode::BAD_REQUEST,
        }
    }

    // fn error_response(&self) ->  HttpResponse {
    //     // HttpResponseBuilder::new(self.status_code())
    //     //     .insert_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
    //     //     .body(self.to_string())
    //     // Error::R
    // }
}


#[derive(Debug)]
pub struct MyError {
   pub name: &'static str,
}

// #[get("/")]
// async fn index() -> Result<&'static str> {
//     let result: Result<&'static str, MyError> = Err(MyError { name: "test error" });
//
//     Ok(result.map_err(|e| error::ErrorBadRequest(e.name))?)
// }
