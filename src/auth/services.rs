use actix_web::{dev::ServiceRequest, web};
use chrono::{Utc, Duration};
use jsonwebtoken::{TokenData, decode, DecodingKey, Validation, encode, Header, EncodingKey};

use crate::{errors::{ServiceError, ServiceErrorType}, config::Config, models::user::User};

use super::claims::TokenClaims;

pub fn validate_token(req: &ServiceRequest, token: &str) -> Result<TokenData<TokenClaims>, ServiceError> {
    let config = req.app_data::<web::Data<Config>>().unwrap();

    let claims = decode::<TokenClaims>(token, &DecodingKey::from_secret(config.jwt_secret.as_ref()), &Validation::default());

    match claims {
        Ok(token_data) => Ok(token_data),
        Err(_) => Err(ServiceError {
            message: Some("le token n'a pas pu être décodé.".to_string()),
            error_type: ServiceErrorType::BadAuthentification
        })
    }
}

pub fn create_valid_token(config: web::Data<Config>, user: &User) -> Result<String, ServiceError> {
    let now = Utc::now();
    let claims = TokenClaims { sub: user.email.clone(), iat: now.timestamp() as usize, exp: (now + Duration::seconds(10)).timestamp() as usize, role: "admin".to_string() };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(config.jwt_secret.as_ref()));

    match token {
        Ok(token_encoded) => Ok(token_encoded),
        Err(_) => Err(ServiceError {
            message: Some("encode failed".to_string()),
            error_type: ServiceErrorType::InternalServerError
        })
    }
}