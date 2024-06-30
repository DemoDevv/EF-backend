use redis::RedisError;
use shared::errors::ServiceError;

#[derive(Debug)]
pub enum SessionError {
    SessionExtractionError,
    GettingSessionError,
    SessionNotFound,
}

impl From<SessionError> for ServiceError {
    fn from(error: SessionError) -> Self {
        match error {
            SessionError::SessionExtractionError => ServiceError {
                message: Some("Failed to extract the session".to_string()),
                error_type: shared::errors::ServiceErrorType::UnAuthorized,
            },
            SessionError::GettingSessionError => ServiceError {
                message: Some("Failed to get the session".to_string()),
                error_type: shared::errors::ServiceErrorType::InternalServerError,
            },
            SessionError::SessionNotFound => ServiceError {
                message: Some("Session not found".to_string()),
                error_type: shared::errors::ServiceErrorType::UnAuthorized,
            },
        }
    }
}

impl Into<actix_web::Error> for SessionError {
    fn into(self) -> actix_web::Error {
        ServiceError::from(self).into()
    }
}

impl From<RedisError> for SessionError {
    fn from(_error: RedisError) -> Self {
        SessionError::GettingSessionError
    }
}
