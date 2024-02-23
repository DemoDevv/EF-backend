use std::fmt;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};

#[derive(Debug)]
pub enum ServiceErrorType {
    BadAuthentification,
    InternalServerError,
    BadDeserialization,
    DatabaseError,
}

#[derive(Debug)]
pub struct ServiceError {
    pub message: Option<String>,
    pub error_type: ServiceErrorType,
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ServiceError {
    fn message(&self) -> String {
        match &self.message {
            Some(message) => message.clone(),
            _ => "An unexpected error has occurred".to_string(),
        }
    }
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self.error_type {
            ServiceErrorType::BadAuthentification => StatusCode::UNAUTHORIZED,
            ServiceErrorType::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceErrorType::BadDeserialization => StatusCode::BAD_REQUEST,
            ServiceErrorType::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(self.message())
    }
}

impl std::convert::From<crate::auth::errors::AuthentificationError> for ServiceError {
    fn from(_error: crate::auth::errors::AuthentificationError) -> Self {
        ServiceError {
            message: Some("Authentification failed".to_string()),
            error_type: ServiceErrorType::BadAuthentification,
        }
    }
}
