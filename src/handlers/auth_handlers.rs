use actix_web::{web, Error, HttpResponse};

use crate::auth::services::create_valid_token;
use crate::config::Config;
use crate::db::repositories::repository::Repository;
use crate::db::connection::Pool;
use crate::errors::{ServiceError, ServiceErrorType};
use crate::extractors::user_extractor::InputUser;
use crate::models::user::User;
use crate::types::roles::Role;
use crate::types::user::NewUser;

pub fn service<R: Repository<User, NewUser>>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/auth")
            .service(web::resource("/login").route(web::post().to(login::<R>)))
            .service(web::resource("/register").route(web::post().to(register::<R>))),
    );
}

pub async fn login<R: Repository<User, NewUser>>(
    config: web::Data<Config>,
    repo: web::Data<R>,
    user: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("login endpoint"))
}

pub async fn register<R: Repository<User, NewUser>>(
    config: web::Data<Config>,
    repo: web::Data<R>,
    user: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    // let user = web::block(move || {
    //     let mut conn = pool.get().expect("couldn't get db connection from pool");

    //     match UsersRepository::find_user_by_email(&mut conn, &user.email) {
    //         Some(_) => Err(ServiceError {
    //             message: Some("l'utilisateur existe déjà".to_string()), // the message has to be more generic
    //             error_type: ServiceErrorType::InternalServerError,
    //         }),
    //         None => Ok(UsersRepository::create_new_user(
    //             &mut conn,
    //             &user.email,
    //             &user.password,
    //         )),
    //     }
    // })
    // .await
    // .map_err(|_| ServiceError {
    //     message: Some("Une erreur c'est produite avec la base de données".to_string()),
    //     error_type: ServiceErrorType::InternalServerError,
    // })?
    // .map_err(|e| e)?;

    let user = repo.create(&NewUser {
        first_name: "Jhon".to_string(),
        last_name: "Doe".to_string(),
        email: user.email.to_string(),
        created_at: chrono::Local::now().naive_local(),
        password: user.password.to_string(),
        salt: "salt_test".to_string(),
        role: Role::User.to_string(),
    }).await?;

    let token = create_valid_token(config, &user)?;

    Ok(HttpResponse::Ok().json(token))
}
