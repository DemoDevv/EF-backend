use actix_web::web;

use crate::handlers::auth::{login, register};
use crate::handlers::health::status;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::resource("/status").route(web::get().to(status))
        ).service(
        web::scope("/api/v1").service(
            web::scope("/auth")
                .service(web::resource("/login").route(web::post().to(login)))
                .service(web::resource("/register").route(web::post().to(register)))
            )
        )
    );
}