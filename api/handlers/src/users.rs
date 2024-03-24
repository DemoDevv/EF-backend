use actix_web::{web, Error, HttpResponse};
use actix_web_httpauth::middleware::HttpAuthentication;

use api_db::repository::UserRepository;
use api_services::auth::middleware::validator;

pub fn service<R: UserRepository>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/users")
            .wrap(HttpAuthentication::bearer(validator))
            .service(
                web::resource("")
                    .route(web::get().to(index::<R>))
                    .route(web::post().to(store::<R>)),
            )
            .service(
                web::resource("/{id}")
                    .route(web::get().to(show::<R>))
                    .route(web::patch().to(update::<R>))
                    .route(web::delete().to(destroy::<R>)),
            ),
    );
}

pub async fn index<R: UserRepository>(repo: web::Data<R>) -> Result<HttpResponse, Error> {
    Ok(repo
        .get_all()
        .await
        .map(|users| HttpResponse::Ok().json(users))?)
}

pub async fn store<R: UserRepository>() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("gestion des données reçus par le formulaire"))
}

pub async fn show<R: UserRepository>() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("One user"))
}

pub async fn update<R: UserRepository>() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("user updated"))
}

pub async fn destroy<R: UserRepository>() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("user removed"))
}
