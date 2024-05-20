use actix_web::{web, Error, HttpResponse};
use api_services::auth::types::Tokens;
use argon2::PasswordHash;
use validator::Validate;

use api_services::auth::services::{create_valid_token, generate_refresh_token};

use api_db::repository::UserRepository;
use api_services::auth::errors::AuthentificationError;
use api_services::auth::helpers::{hash_password, verify_password};
use api_services::redis::{RedisClient, RedisRepository};
use api_types::roles::Role;
use api_types::user::NewUser;
use api_types::user::{InputUser, RefreshableUser};
use shared::config::Config;
use shared::errors::{ServiceError, ServiceErrorType};

pub fn service<R: UserRepository>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/auth")
            .service(web::resource("/login").route(web::post().to(login::<R>)))
            .service(web::resource("/register").route(web::post().to(register::<R>)))
            .service(web::resource("/refresh").route(web::post().to(refresh_tokens::<R>))),
    );
}

pub async fn login<R: UserRepository>(
    config: web::Data<Config>,
    repo: web::Data<R>,
    redis_client: web::Data<RedisClient>,
    user_json: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    // we make the validation of the user entry
    user_json.validate().map_err(|err| ServiceError {
        message: Some(format!("Invalid user: {}", err)),
        error_type: ServiceErrorType::BadDeserialization,
    })?;

    let generique_error = ServiceError {
        // TODO: make this error in a const file
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

    let tokens = Tokens {
        access_token: create_valid_token(config.clone(), &user)?,
        refresh_token: generate_refresh_token(),
    };

    // TODO: changer le ttl en fonction de la configuration
    redis_client
        .update_ttl(
            &tokens.refresh_token,
            &user.id.to_string(),
            config.refresh_token_ttl,
        )
        .await
        .map_err(|err| ServiceError::from(err))?;

    Ok(HttpResponse::Ok().json(tokens))
}

pub async fn register<R: UserRepository>(
    config: web::Data<Config>,
    repo: web::Data<R>,
    redis_client: web::Data<RedisClient>,
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

    let tokens = Tokens {
        access_token: create_valid_token(config.clone(), &user)?,
        refresh_token: generate_refresh_token(),
    };

    redis_client
        .update_ttl(
            &tokens.refresh_token,
            &user.id.to_string(),
            config.refresh_token_ttl,
        )
        .await
        .map_err(|err| ServiceError::from(err))?;

    Ok(HttpResponse::Ok().json(tokens))
}

pub async fn refresh_tokens<R: UserRepository>(
    config: web::Data<Config>,
    user: web::Json<RefreshableUser>,
    redis_client: web::Data<RedisClient>,
    user_repository: web::Data<R>,
) -> Result<HttpResponse, Error> {
    // créer un nouveau access token et un refresh token puis modifier le refresh token dans redis
    user.validate().map_err(|err| ServiceError {
        message: Some(format!("Invalid user: {}", err)),
        error_type: ServiceErrorType::BadDeserialization,
    })?;

    let id_user = match redis_client.get(&user.refresh_token).await {
        Ok(Some(id)) => id,
        Ok(None) => {
            return Err(ServiceError {
                message: Some("Refresh token not found".to_string()),
                error_type: ServiceErrorType::UnAuthorized,
            }
            .into())
        }
        Err(err) => return Err(ServiceError::from(err).into()),
    }
    .parse::<i32>()
    .unwrap();

    let user_from_db = user_repository.get(id_user).await?;

    if user_from_db.email != user.email {
        return Err(ServiceError {
            message: Some("Email does not match".to_string()),
            error_type: ServiceErrorType::UnAuthorized,
        }
        .into());
    }

    let tokens = Tokens {
        access_token: create_valid_token(config.clone(), &user_from_db)?,
        refresh_token: generate_refresh_token(),
    };

    redis_client
        .delete(&user.refresh_token)
        .await
        .map_err(|err| ServiceError::from(err))?;
    redis_client
        .update_ttl(
            &tokens.refresh_token,
            &id_user.to_string(),
            config.refresh_token_ttl,
        )
        .await
        .map_err(|err| ServiceError::from(err))?;

    Ok(HttpResponse::Ok().json(tokens))
}
