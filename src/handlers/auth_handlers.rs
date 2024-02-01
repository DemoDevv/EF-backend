use actix_web::{web, Error, HttpResponse};

use crate::auth::services::create_valid_token;
use crate::config::Config;
use crate::db::{connection::Pool, repositories::users_repository::UsersRepository};
use crate::errors::{ServiceError, ServiceErrorType};
use crate::extractors::user_extractor::InputUser;

pub async fn login(
    config: web::Data<Config>,
    pool: web::Data<Pool>,
    user: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("login endpoint"))
}

pub async fn register(
    config: web::Data<Config>,
    pool: web::Data<Pool>,
    user: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        match UsersRepository::find_user_by_email(&mut conn, &user.email) {
            Some(_) => Err(ServiceError {
                message: Some("l'utilisateur existe déjà".to_string()), // the message has to be more generic
                error_type: ServiceErrorType::InternalServerError,
            }),
            None => Ok(UsersRepository::create_new_user(
                &mut conn,
                &user.email,
                &user.password,
            )),
        }
    })
    .await
    .map_err(|_| ServiceError {
        message: Some("Une erreur c'est produite avec la base de données".to_string()),
        error_type: ServiceErrorType::InternalServerError,
    })?
    .map_err(|e| {
        e
    })?;
    
    // TODO: faire un sleep de 3 secondes si quelque chose n'est pas correct

    let token = create_valid_token(config, &user)?;

    Ok(HttpResponse::Ok().json(token))
}
