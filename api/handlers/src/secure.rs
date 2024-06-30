use actix_web::{web, Error, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;

use api_extractors::session::Session;
use api_services::auth::middleware::validator;
use api_services::auth::services::decode_token;
use shared::config::Config;

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/secure")
            .service(
                web::scope("/test-session")
                    .wrap(api_middlewares::redis_session_middleware::RedisSessionMiddlewareFactory::new(
                        api_services::redis::get_redis_client(&shared::CONFIG),
                    ))
                    .service(web::resource("").route(web::get().to(test_session))),
            )
            .service(
                web::scope("/test-jwt")
                    .wrap(HttpAuthentication::bearer(validator))
                    .service(web::resource("").route(web::get().to(test))),
            ),
    );
}

pub async fn test(config: web::Data<Config>, token: BearerAuth) -> Result<HttpResponse, Error> {
    // j'unwrap car je sais que le token est valide étant donné que j'ai passé le middleware
    let claims = decode_token(config, token.token()).unwrap().claims;

    Ok(HttpResponse::Ok().json("good token, you have the role: ".to_string() + &claims.role))
}

// TODO: manque des tests
pub async fn test_session(session: Session) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("good session, you have the email: ".to_string() + &session.email))
}
