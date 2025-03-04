use actix_web::{web, Error, HttpResponse};
use actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication};

use api_configs::config::Config;
use api_db::{models::user::User, repository::UserRepository};
use api_model_traits::update::Updatable;
use api_services::{
    auth::{middleware::validator, services::decode_token},
    users::UsersService,
};
use api_types::user::{NewUserWithId, UpdatableUser, UserPayload};

pub fn service<R: UserRepository>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/users")
            .wrap(HttpAuthentication::bearer(validator))
            .service(web::resource("/profile").route(web::get().to(profile)))
            .service(
                web::resource("/{id}")
                    .route(web::get().to(show))
                    .route(web::patch().to(update::<R>))
                    .route(web::put().to(replace::<R>))
                    .route(web::delete().to(destroy)),
            ),
    );
}

pub async fn profile(
    user_service: web::Data<UsersService>,
    config: web::Data<Config>,
    token: BearerAuth,
) -> Result<HttpResponse, Error> {
    let claims = decode_token(config, token.token()).unwrap().claims;

    Ok(user_service
        .get_safe_user(claims.sub)
        .await
        .map(|user| HttpResponse::Ok().json(user))?)
}

/// This function is used to show a user from the database
pub async fn show(
    user_service: web::Data<UsersService>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(user_service
        .get_safe_user(id.into_inner())
        .await
        .map(|user| HttpResponse::Ok().json(user))?)
}

/// This function is used to update partial content of a user from the database
pub async fn update<R: UserRepository>(
    repository: web::Data<R>,
    id: web::Path<i32>,
    updatable_user: web::Json<UpdatableUser>,
) -> Result<HttpResponse, Error> {
    // il faut pouvoir transformer UpdatableUser en User car il dispose de asChangeSet
    let user = repository
        .get(id.into_inner())
        .await?
        .perform_update(updatable_user.into_inner())?;

    let updated_user = repository.update(user.id, &user).await?;

    Ok(HttpResponse::Ok().json(updated_user))
}

/// This function is used to update a user from the database
pub async fn replace<R: UserRepository>(
    repository: web::Data<R>,
    id: web::Path<i32>,
    user_payload: web::Json<UserPayload>,
) -> Result<HttpResponse, Error> {
    let user = User::from(NewUserWithId {
        id: *id,
        user: user_payload.into_inner(),
    });

    let updated_user = repository.update(id.into_inner(), &user).await?;

    Ok(HttpResponse::Ok().json(updated_user))
}

/// This function is used to delete a user from the database
pub async fn destroy(
    user_service: web::Data<UsersService>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(user_service
        .destroy_user(id.into_inner())
        .await
        .map(|_| HttpResponse::Ok().json("Delete action perfomed on the user"))?)
}
