use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use super::services::validate_token;

/// Validate the token and insert the user id in the request extensions.
/// Used in the authentication middleware.
pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    match validate_token(&req, credentials.token()) {
        Ok(token_data) => {
            // we give in the request extension the user id for use it in middleware
            req.extensions_mut().insert(token_data.claims.sub);
            Ok(req)
        }
        Err(err) => Err((err.into(), req)),
    }
}
