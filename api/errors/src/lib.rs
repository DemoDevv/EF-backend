extern crate redis;

use std::fmt;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};

#[derive(Debug, Eq, PartialEq)]
pub enum ServiceErrorType {
    BadAuthentification,
    InternalServerError,
    BadDeserialization,
    DatabaseError,
    UnAuthorized,
    UnprocessableEntityError,
    Conflict,
    NotFound,
    RateLimitExceeded,
}

#[derive(Debug, Eq, PartialEq)]
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
            ServiceErrorType::UnAuthorized => StatusCode::UNAUTHORIZED,
            ServiceErrorType::UnprocessableEntityError => StatusCode::UNPROCESSABLE_ENTITY,
            ServiceErrorType::Conflict => StatusCode::CONFLICT,
            ServiceErrorType::NotFound => StatusCode::NOT_FOUND,
            ServiceErrorType::RateLimitExceeded => StatusCode::TOO_MANY_REQUESTS,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(self.message())
    }
}

impl From<api_caches::errors::RedisRepositoryError> for ServiceError {
    fn from(error: api_caches::errors::RedisRepositoryError) -> Self {
        ServiceError {
            message: Some(error.to_string()),
            error_type: ServiceErrorType::DatabaseError,
        }
    }
}

impl From<api_caches::errors::RateLimitError> for ServiceError {
    fn from(error: api_caches::errors::RateLimitError) -> Self {
        ServiceError {
            message: Some(error.to_string()),
            error_type: ServiceErrorType::RateLimitExceeded,
        }
    }
}

impl From<reqwest::Error> for ServiceError {
    fn from(_error: reqwest::Error) -> Self {
        ServiceError {
            message: Some("Reqwest error".to_string()),
            error_type: ServiceErrorType::InternalServerError,
        }
    }
}
