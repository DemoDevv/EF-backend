use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use super::services::validate_token;

pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    match validate_token(&req, credentials.token()) {
        Ok(_) => Ok(req), // on peut utiliser ce qui ce trouve dans les claims ici
        Err(err) => Err((err.into(), req))
    }
}