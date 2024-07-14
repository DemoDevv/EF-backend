use actix_web::{dev::ServiceRequest, web};

use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use time::{Duration, OffsetDateTime};

use api_configs::config::Config;
use api_db::models::user::User;
use api_errors::{ServiceError, ServiceErrorType};

use super::claims::TokenClaims;

pub fn validate_token(
    req: &ServiceRequest,
    token: &str,
) -> Result<TokenData<TokenClaims>, ServiceError> {
    let config = req.app_data::<web::Data<Config>>().unwrap();

    let claims = decode_token(config.clone(), token);

    match claims {
        Ok(token_data) => Ok(token_data),
        Err(_) => Err(ServiceError {
            message: Some("le token n'a pas pu être décodé.".to_string()),
            error_type: ServiceErrorType::BadAuthentification,
        }),
    }
}

pub fn create_valid_token(config: &Config, user: &User) -> Result<String, ServiceError> {
    let iat = OffsetDateTime::now_utc();
    let exp = iat + Duration::minutes(config.jwt_expired_in);

    let claims = TokenClaims::new(user.email.clone(), iat, exp, user.role.clone());

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    );

    match token {
        Ok(token_encoded) => Ok(token_encoded),
        Err(_) => Err(ServiceError {
            message: Some("encode failed".to_string()),
            error_type: ServiceErrorType::InternalServerError,
        }),
    }
}

pub fn decode_token(
    config: web::Data<Config>,
    token: &str,
) -> Result<TokenData<TokenClaims>, jsonwebtoken::errors::Error> {
    // suppresion du leeway pour pouvoir exp le token avant 1 minute
    let mut validation = Validation::new(Algorithm::HS256);
    validation.leeway = 1;

    decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(config.jwt_secret.as_ref()),
        &validation,
    )
}

pub fn generate_refresh_token() -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}
