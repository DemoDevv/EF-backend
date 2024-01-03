use actix_web::{Error, HttpResponse, web};

use crate::db::{connection::Pool, repositories::users::UsersRepository};
use crate::extractors::user::InputUser;
use crate::errors::{ServiceError, ServiceErrorType};
use crate::config::Config;
use crate::auth::services::create_valid_token;

pub async fn login(config: web::Data<Config>, pool: web::Data<Pool>, user: web::Json<InputUser>) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        UsersRepository::create_new_user(&mut conn, &user.email, &user.password)
    })
    .await
    .map(|user| user)
    .map_err(|_| ServiceError {message: Some("l'utilisateur n'a pas pu être inséré".to_string()), error_type: ServiceErrorType::InternalServerError})?;

    let token = create_valid_token(config, &user)?;

    Ok(HttpResponse::Ok().json(token))
}

pub async fn register() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("register endpoint"))
}