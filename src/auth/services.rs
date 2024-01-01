use actix_web::{dev::ServiceRequest, web};
use jsonwebtoken::{TokenData, decode, DecodingKey, Validation};

use crate::{AppState, errors::{ServiceError, ServiceErrorType}};

use super::claims::TokenClaims;

pub fn validate_token(req: &ServiceRequest, token: &str) -> Result<TokenData<TokenClaims>, ServiceError> {
    let data = req.app_data::<web::Data<AppState>>().unwrap();

    let claims = decode::<TokenClaims>(token, &DecodingKey::from_secret(data.env.jwt_secret.as_ref()), &Validation::default());

    match claims {
        Ok(token_data) => Ok(token_data),
        Err(_) => Err(ServiceError {
            cause: None,
            message: Some("le token n'a pas pu être décodé.".to_string()),
            error_type: ServiceErrorType::BadAuthentification
        })
    }
}