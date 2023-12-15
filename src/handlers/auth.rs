use actix_web::{Error, HttpResponse};

pub async fn login() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("login endpoint"))
}

pub async fn register() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("register endpoint"))
}