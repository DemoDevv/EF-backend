use shared::errors::ServiceError;

#[derive(Debug)]
pub enum AuthentificationError {
    IncorrectPassword,
    Argon2Error(argon2::password_hash::Error),
}

impl From<argon2::password_hash::Error> for AuthentificationError {
    fn from(err: argon2::password_hash::Error) -> Self {
        AuthentificationError::Argon2Error(err)
    }
}

impl From<AuthentificationError> for ServiceError {
    fn from(_error: AuthentificationError) -> Self {
        ServiceError {
            message: Some("Authentification failed".to_string()),
            error_type: shared::errors::ServiceErrorType::BadAuthentification,
        }
    }
}
