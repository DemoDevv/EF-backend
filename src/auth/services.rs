use actix_web::{dev::ServiceRequest, web};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use time::{Duration, OffsetDateTime};

use crate::{errors::{ServiceError, ServiceErrorType}, config::Config, models::user::User};

use super::claims::TokenClaims;

pub fn validate_token(req: &ServiceRequest, token: &str) -> Result<TokenData<TokenClaims>, ServiceError> {
    let config = req.app_data::<web::Data<Config>>().unwrap();

    // suppresion du leeway pour pouvoir exp le token avant 1 minute
    let mut validation = Validation::new(Algorithm::HS256);
    validation.leeway = 1;

    let claims = decode::<TokenClaims>(&token, &DecodingKey::from_secret(config.jwt_secret.as_ref()), &validation);

    match claims {
        Ok(token_data) => Ok(token_data),
        Err(_) => {
            Err(ServiceError {
                message: Some("le token n'a pas pu être décodé.".to_string()),
                error_type: ServiceErrorType::BadAuthentification
            })
        }
    }
}

pub fn create_valid_token(config: web::Data<Config>, user: &User) -> Result<String, ServiceError> {
    let iat = OffsetDateTime::now_utc();
    let exp = iat + Duration::seconds(25);

    let claims = TokenClaims::new(user.email.clone(), iat, exp, "admin".to_string());

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(config.jwt_secret.as_ref()));

    match token {
        Ok(token_encoded) => Ok(token_encoded),
        Err(_) => Err(ServiceError {
            message: Some("encode failed".to_string()),
            error_type: ServiceErrorType::InternalServerError
        })
    }
}