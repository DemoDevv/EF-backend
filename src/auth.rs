use actix_web::{dev::ServiceRequest, web, Error};
use actix_web_httpauth::extractors::{bearer::{BearerAuth, Config}, AuthenticationError};
use jsonwebtoken::{decode, DecodingKey, Validation, TokenData};
use serde::{Deserialize, Serialize};

use crate::{AppState, errors::ServiceError};

#[derive(Deserialize, Serialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

fn validate_token(req: &ServiceRequest, token: &str) -> Result<TokenData<TokenClaims>, ServiceError> {
    let data = req.app_data::<web::Data<AppState>>().unwrap();

    let claims = decode::<TokenClaims>(token, &DecodingKey::from_secret(data.env.jwt_secret.as_ref()), &Validation::default())?;

    Ok(claims)
}

pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    match validate_token(&req, credentials.token()) {
        Ok(token_data) => Ok(req), // on peut utiliser ce qui ce trouve dans les claims ici
        Err(_) => {
            let config = req.app_data::<Config>().cloned().unwrap_or_default();
            Err((AuthenticationError::from(config).into(), req))
        }
    }
}