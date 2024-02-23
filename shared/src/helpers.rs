use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    }, Argon2, PasswordHash, PasswordVerifier
};

pub fn hash_password(password: &str) -> Result<String, crate::auth::errors::AuthentificationError> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();

    Ok(password_hash)
}

pub fn verify_password(password: &str, hash: &PasswordHash) -> Result<bool, crate::auth::errors::AuthentificationError> {
    let argon2 = Argon2::default();

    argon2.verify_password(password.as_bytes(), hash).map_err(|err| match err {
        argon2::password_hash::Error::Password => crate::auth::errors::AuthentificationError::IncorrectPassword,
        _ => crate::auth::errors::AuthentificationError::Argon2Error(err),
    })?;

    Ok(true)
}