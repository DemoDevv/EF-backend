use actix_web::{web, Error, HttpResponse};

use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;

use validator::Validate;

use api_caches::access_refresh_tokens::AccessRefreshTokensCache;
use api_configs::config::Config;
use api_db::repository::UserRepository;
use api_errors::{ServiceError, ServiceErrorType};
use api_services::auth::middleware::validator;
use api_services::auth::services::AuthService;
use api_types::user::{InputUser, RefreshableUser};

use crate::helpers::tokens::send_secure_tokens;

pub fn service<U: UserRepository, C: AccessRefreshTokensCache>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/auth")
            .service(web::resource("/login").route(web::post().to(login::<U, C>)))
            .service(web::resource("/register").route(web::post().to(register::<U, C>)))
            .service(web::resource("/refresh").route(web::post().to(refresh_tokens::<U, C>)))
            .service(
                web::scope("/token")
                    .wrap(HttpAuthentication::bearer(validator))
                    .service(web::resource("/verify").route(web::get().to(verify_token))),
            ),
    );
}

pub async fn login<U: UserRepository, C: AccessRefreshTokensCache>(
    config: web::Data<Config>,
    auth_service: web::Data<AuthService<U, C>>,
    user_json: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    user_json.validate().map_err(|err| ServiceError {
        message: Some(format!("Invalid user: {}", err)),
        error_type: ServiceErrorType::BadDeserialization,
    })?;

    let tokens = auth_service.login(user_json.into_inner(), &config).await?;

    Ok(send_secure_tokens(tokens, &config))
}

pub async fn register<U: UserRepository, C: AccessRefreshTokensCache>(
    config: web::Data<Config>,
    auth_service: web::Data<AuthService<U, C>>,
    user_json: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    user_json.validate().map_err(|err| ServiceError {
        message: Some(format!("Invalid user: {}", err)),
        error_type: ServiceErrorType::BadDeserialization,
    })?;

    let tokens = auth_service
        .register(user_json.into_inner(), &config)
        .await?;

    Ok(send_secure_tokens(tokens, &config))
}

pub async fn verify_token(
    _config: web::Data<Config>,
    _token: BearerAuth,
) -> Result<HttpResponse, Error> {
    // TODO: for the moment we use the bearer middleware
    // i think we have to respond with an other code error (not 401)
    Ok(HttpResponse::Ok().finish())
}

pub async fn refresh_tokens<U: UserRepository, C: AccessRefreshTokensCache>(
    config: web::Data<Config>,
    user: web::Json<RefreshableUser>,
    auth_service: web::Data<AuthService<U, C>>,
) -> Result<HttpResponse, Error> {
    // créer un nouveau access token et un refresh token puis modifier le refresh token dans redis
    user.validate().map_err(|err| ServiceError {
        message: Some(format!("Invalid user: {}", err)),
        error_type: ServiceErrorType::BadDeserialization,
    })?;

    let tokens = auth_service
        .refresh_tokens(user.into_inner(), &config)
        .await?;

    Ok(send_secure_tokens(tokens, &config))
}
