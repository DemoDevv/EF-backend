use actix_web::{web, Error, HttpResponse};

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/status").route(web::get().to(status)));
}

pub async fn status() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("the server is alive."))
}
