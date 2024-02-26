use actix_web::{web, Error, HttpResponse};

use shared::config::Config;

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/status")
            .wrap(api_middlewares::example::TestMiddlewareFactory::new())
            .route(web::get().to(status)),
    );
}

pub async fn status(config: web::Data<Config>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .append_header(("VERSION", config.version.to_string()))
        .json("the server is alive."))
}
