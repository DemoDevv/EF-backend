use shared::errors::ServiceError;

#[derive(Debug)]
pub enum SessionError {
    SessionExtractionError,
}

impl From<SessionError> for ServiceError {
    fn from(_error: SessionError) -> Self {
        ServiceError {
            message: Some(
                "Failed to extract the session. Maybe the user is not authenticated. ".to_string(),
            ),
            error_type: shared::errors::ServiceErrorType::UnAuthorized,
        }
    }
}

impl Into<actix_web::Error> for SessionError {
    fn into(self) -> actix_web::Error {
        ServiceError::from(self).into()
    }
}
