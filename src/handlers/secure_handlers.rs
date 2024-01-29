use actix_web::{Error, HttpResponse};

pub async fn test() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("good token"))
}