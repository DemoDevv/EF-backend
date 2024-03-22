use actix_web::{web, Error, HttpResponse};
use actix_web_httpauth::middleware::HttpAuthentication;

use api_services::auth::middleware::validator;

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/users")
            .wrap(HttpAuthentication::bearer(validator))
            .service(
                web::resource("")
                    .route(web::get().to(index))
                    .route(web::post().to(store)),
            )
            .service(
                web::resource("/{id}")
                    .route(web::get().to(show))
                    .route(web::patch().to(update))
                    .route(web::delete().to(destroy)),
            ),
    );
}

pub async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("all users"))
}

pub async fn store() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("gestion des données reçus par le formulaire"))
}

pub async fn show() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("One user"))
}

pub async fn update() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("user updated"))
}

pub async fn destroy() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("user removed"))
}
