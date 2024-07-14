use actix_web::{web, Error, HttpResponse};

use api_configs::config::Config;

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/status").route(web::get().to(status)));
}

pub async fn status(config: web::Data<Config>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .append_header(("VERSION", config.version.to_string()))
        .json("the server is alive."))
}
