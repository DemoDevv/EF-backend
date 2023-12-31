use std::fmt;

use actix_web::{ResponseError, http::StatusCode, HttpResponse};

#[derive(Debug)]
pub enum ServiceErrorType {
    BadAuthentification,
}

#[derive(Debug)]
pub struct ServiceError {
    pub cause: Option<String>,
    pub message: Option<String>,
    pub error_type: ServiceErrorType
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

impl From<jsonwebtoken::errors::Error> for ServiceError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        ServiceError {
            message: None,
            cause: Some(error.to_string()),
            error_type: ServiceErrorType::BadAuthentification
        }
    }
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self.error_type {
            ServiceErrorType::BadAuthentification => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(
            self.message()
        )
    }
}