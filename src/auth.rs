use actix_web::{dev::ServiceRequest, Error, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Deserialize, Serialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

fn validate_token(req: &ServiceRequest, token: &str) -> bool {
    let data = req.app_data::<web::Data<AppState>>().unwrap();

    let claims = match decode::<TokenClaims>(token, &DecodingKey::from_secret(data.env.jwt_secret.as_ref()), &Validation::default()) {
        Ok(data) => data.claims,
        Err(_) => todo!(),
    };

    true
}

pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    match validate_token(&req, credentials.token()) {
        true => Ok(req),
        false => todo!()
    }
}