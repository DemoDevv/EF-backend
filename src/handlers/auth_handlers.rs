use actix_web::{web, Error, HttpResponse};

use crate::auth::services::create_valid_token;
use crate::config::Config;
use crate::db::repositories::repository::UserRepository;
use crate::errors::{ServiceError, ServiceErrorType};
use crate::extractors::user_extractor::InputUser;
use crate::types::roles::Role;
use crate::types::user::NewUser;

pub fn service<R: UserRepository>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/auth")
            .service(web::resource("/login").route(web::post().to(login::<R>)))
            .service(web::resource("/register").route(web::post().to(register::<R>))),
    );
}

pub async fn login<R: UserRepository>(
    config: web::Data<Config>,
    repo: web::Data<R>,
    user: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("login endpoint"))
}

pub async fn register<R: UserRepository>(
    config: web::Data<Config>,
    repo: web::Data<R>,
    user: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    let user = match repo.get_user_by_email(&user.email).await {
        Ok(_) => Err(ServiceError {
            message: Some("l'utilisateur existe déjà".to_string()), // the message has to be more generic
            error_type: ServiceErrorType::InternalServerError,
        }),
        // FIXME: the error can be different of the not found error
        Err(_) => {
            repo.create(&NewUser {
                first_name: "Jhon".to_string(),
                last_name: "Doe".to_string(),
                email: user.email.to_string(),
                created_at: chrono::Local::now().naive_local(),
                password: user.password.to_string(),
                salt: "salt_test".to_string(),
                role: Role::User.to_string(),
            })
            .await
        }
    }?;

    let token = create_valid_token(config, &user)?;

    Ok(HttpResponse::Ok().json(token))
}
