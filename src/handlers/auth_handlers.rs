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
        UsersRepository::create_new_user(&mut conn, &user.email, &user.password)
    })
    .await
    .map(|user| user)
    .map_err(|_| ServiceError {
        message: Some("l'utilisateur n'a pas pu être inséré".to_string()),
        error_type: ServiceErrorType::InternalServerError,
    })?;

    let token = create_valid_token(config, &user)?;

    Ok(HttpResponse::Ok().json(token))
}
