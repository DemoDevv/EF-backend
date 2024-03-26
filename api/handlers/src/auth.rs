use actix_web::{web, Error, HttpResponse};
use api_services::auth::types::Tokens;
use argon2::PasswordHash;
use validator::Validate;

use api_services::auth::services::{create_valid_token, generate_refresh_token};

use api_db::repository::UserRepository;
use api_services::auth::errors::AuthentificationError;
use api_services::auth::helpers::{hash_password, verify_password};
use shared::config::Config;
use shared::errors::{ServiceError, ServiceErrorType};
use shared::extractors::user_extractor::InputUser;
use shared::types::roles::Role;
use shared::types::user::NewUser;

pub fn service<R: UserRepository>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/auth")
            .service(web::resource("/login").route(web::post().to(login::<R>)))
            .service(web::resource("/register").route(web::post().to(register::<R>)))
            .service(web::resource("/refresh").route(web::post().to(refresh_tokens))),
    );
}

pub async fn login<R: UserRepository>(
    config: web::Data<Config>,
    repo: web::Data<R>,
    user_json: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    // we make the validation of the user entry
    user_json.validate().map_err(|err| ServiceError {
        message: Some(format!("Invalid user: {}", err)),
        error_type: ServiceErrorType::BadDeserialization,
    })?;

    let generique_error = ServiceError {
        message: Some("Authentification failed".to_string()),
        error_type: ServiceErrorType::BadAuthentification,
    };

    let user = match repo.get_user_by_email(&user_json.email).await {
        Ok(user_from_the_db) => {
            // we parse the hash from the database
            let parsed_hash = PasswordHash::new(&user_from_the_db.password)
                .map_err(|err| AuthentificationError::from(err))
                .map_err(|err| ServiceError::from(err))?;

            // we verify the hash with the hash from the user
            verify_password(&user_json.password, &parsed_hash)
                .map_err(|err| ServiceError::from(err))?;

            // we don't have error so we return the user
            Ok(user_from_the_db)
        }
        Err(_) => Err(generique_error),
    }?;

    let token = create_valid_token(config, &user)?;
    let refresh_token = generate_refresh_token();

    let tokens = Tokens {
        access_token: token,
        refresh_token,
    };

    // TODO: set dans redis key/value avec la key = refresh_token et value = user.id

    Ok(HttpResponse::Ok().json(tokens))
}

pub async fn register<R: UserRepository>(
    config: web::Data<Config>,
    repo: web::Data<R>,
    user_json: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    // we make the validation of the user entry
    user_json.validate().map_err(|err| ServiceError {
        message: Some(format!("Invalid user: {}", err)),
        error_type: ServiceErrorType::BadDeserialization,
    })?;

    // FIXME: timing attack possible
    let user = match repo.get_user_by_email(&user_json.email).await {
        Ok(_) => Err(ServiceError {
            message: Some("User already exist".to_string()), // the message has to be more generic
            error_type: ServiceErrorType::InternalServerError,
        }),
        // the error can be different of the not found error and in this case we don't want to create the user
        Err(e) => {
            match e.error_type {
                ServiceErrorType::DatabaseError => Err(ServiceError {
                    message: Some("Error getting user".to_string()),
                    error_type: ServiceErrorType::InternalServerError,
                }),
                _ => {
                    // password hashing
                    let hash = hash_password(&user_json.password)
                        .map_err(|err| ServiceError::from(err))?;

                    repo.create(&NewUser {
                        first_name: "Jhon".to_string(),
                        last_name: "Doe".to_string(),
                        email: user_json.email.to_string(),
                        created_at: chrono::Local::now().naive_local(),
                        password: hash,
                        role: Role::User.to_string(),
                    })
                    .await
                }
            }
        }
    }?;

    let token = create_valid_token(config, &user)?;
    let refresh_token = generate_refresh_token();

    let tokens = Tokens {
        access_token: token,
        refresh_token,
    };

    Ok(HttpResponse::Ok().json(tokens))
}

pub async fn refresh_tokens() -> Result<HttpResponse, Error> {
    // créer un nouveau access token et un refresh token puis modifier le refresh token dans redis
    Ok(HttpResponse::Ok().finish())
}
