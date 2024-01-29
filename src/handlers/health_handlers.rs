use actix_web::{Error, HttpResponse};

pub async fn status() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("the server is alive."))
}