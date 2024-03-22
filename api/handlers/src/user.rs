use actix_web::{web, Error, HttpResponse};
use actix_web_httpauth::middleware::HttpAuthentication;

use api_services::auth::middleware::validator;

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/users")
            .wrap(HttpAuthentication::bearer(validator))
            .service(web::resource("/").route(web::get().to(index))),
    );
}

pub async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("all users"))
}