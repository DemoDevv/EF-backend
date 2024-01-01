use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::handlers::auth::{login, register};
use crate::handlers::health::status;
use crate::auth::middleware::validator;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
        .service(
            web::resource("/status").route(web::get().to(status))
        )
        .service(
            web::scope("/v1/auth")
            .service(web::resource("/login").route(web::post().to(login)))
            .service(web::resource("/register").route(web::post().to(register)))
        )
        .service(
            web::scope("/secure")
            .wrap(HttpAuthentication::bearer(validator))
        )
    );
}