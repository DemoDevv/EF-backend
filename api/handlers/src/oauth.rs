use actix_web::{http::header, web, Error, HttpResponse};

use serde::Deserialize;

use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthorizationCode, CsrfToken, Scope,
    TokenResponse,
};

use api_configs::config::Config;
use api_services::auth::types::Tokens;

use crate::helpers::tokens::send_secure_tokens;

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/oauth").service(
            web::scope("/google")
                .service(web::resource("/login").route(web::get().to(login)))
                .service(web::resource("/oauth2callback").route(web::get().to(oauth2callback))),
        ),
    );
}

pub async fn login(oauth_client: web::Data<BasicClient>) -> Result<HttpResponse, Error> {
    let (authorization_url, _crf_token) = oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_extra_param("access_type", "offline")
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.email".to_string(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.profile".to_string(),
        ))
        .url();

    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, authorization_url.to_string()))
        .finish())
}

#[derive(Deserialize)]
pub struct AuthRequest {
    code: String,
}

pub async fn oauth2callback(
    params: web::Query<AuthRequest>,
    oauth_client: web::Data<BasicClient>,
    config: web::Data<Config>,
) -> Result<HttpResponse, Error> {
    let code = AuthorizationCode::new(params.code.clone());

    let token_result = oauth_client
        .exchange_code(code)
        .request_async(async_http_client)
        .await;

    match token_result {
        Ok(token) => {
            if let Some(refresh_token) = token.refresh_token() {
                Ok(send_secure_tokens(
                    Tokens {
                        access_token: token.access_token().secret().to_string(),
                        refresh_token: refresh_token.secret().to_string(),
                    },
                    &config,
                ))
            } else {
                Ok(HttpResponse::Ok().json(token.access_token().secret().to_string()))
            }
        }
        Err(_) => Ok(HttpResponse::BadRequest().json("Échec de l'authentification")),
    }
}
