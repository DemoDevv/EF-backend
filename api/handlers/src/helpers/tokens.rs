use actix_web::{cookie::Cookie, HttpResponse};
use api_services::auth::types::Tokens;
use shared::config::Config;

pub(crate) fn send_secure_tokens(tokens: Tokens, config: &Config) -> HttpResponse {
    HttpResponse::Ok()
        .cookie(
            Cookie::build("refresh_token", tokens.refresh_token.clone())
                .http_only(true)
                .secure(true)
                .same_site(actix_web::cookie::SameSite::Strict)
                .path("/")
                .max_age(actix_web::cookie::time::Duration::days(
                    config.refresh_token_ttl,
                ))
                .finish(),
        )
        .json(tokens)
}
