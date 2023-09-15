//! router setting
use actix_web::web;
use crate::{
    middleware,
    handler::{basic, user, course},
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        // .wrap(middleware::ReadReqBody)
        // .wrap(middleware::Jwt)
        // // .wrap(middleware::AccessLogging)
        // .wrap(middleware::AccessLogging::default().log_target("http_log"))
        // .app_data(counter.clone()) // <- register the created data
        .service(basic::index)

        .service(
            // /app
            web::scope("/app")
                .wrap(middleware::Jwt)
                // .route("/user", web::post().to(user::user_handler))
                .route("/greet", web::get().to(basic::greet))
                .route("/state", web::post().to(basic::state))
                .service(user::bytes_handler)
                .service(user::extract_json_handler)
                .service(user::json_handler)
                .service(user::payload_handler)
                .service(course::get_courses)
                .service(course::add_courses)
                .service(course::del_courses)
                .service(course::update_courses),
        );
}
