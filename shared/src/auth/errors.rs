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
