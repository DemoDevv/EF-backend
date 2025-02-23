use actix_web::{web, Error, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;

use api_configs::config::Config;
use api_services::auth::middleware::validator;
use api_services::auth::services::decode_token;

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/secure").service(
            web::scope("/test-jwt")
                .wrap(HttpAuthentication::bearer(validator))
                .service(web::resource("").route(web::get().to(test))),
        ),
    );
}

pub async fn test(config: web::Data<Config>, token: BearerAuth) -> Result<HttpResponse, Error> {
    // we can unwrap here because the token is already validated
    let _ = decode_token(config, token.token()).unwrap().claims;

    Ok(HttpResponse::Ok().into())
}
