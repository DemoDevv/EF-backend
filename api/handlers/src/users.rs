use actix_web::{web, Error, HttpResponse};
use actix_web_httpauth::middleware::HttpAuthentication;

use api_db::{models::user::User, repository::UserRepository};
use api_services::auth::middleware::validator;
use shared::{extractors::user_extractor::UpdatableUser, types::user::{NewUser, NewUserWithId, SafeUser}};

pub fn service<R: UserRepository>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/users")
            .wrap(HttpAuthentication::bearer(validator))
            .service(
                web::resource("")
                    .route(web::get().to(index::<R>))
                    .route(web::post().to(store::<R>)),
            )
            .service(
                web::resource("/{id}")
                    .route(web::get().to(show::<R>))
                    .route(web::patch().to(update::<R>))
                    .route(web::put().to(replace::<R>))
                    .route(web::delete().to(destroy::<R>)),
            ),
    );
}

/// This function is used to get all entities from the database
pub async fn index<R: UserRepository>(repository: web::Data<R>) -> Result<HttpResponse, Error> {
    Ok(repository
        .get_all()
        .await
        .map(|users| users.into_iter().map(|user| user.into()).collect::<Vec<SafeUser>>())
        .map(|users| HttpResponse::Ok().json(users))?)
}

/// This function is used to store a new user in the database with the data received from the form
pub async fn store<R: UserRepository>() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("gestion des données reçus par le formulaire"))
}

/// This function is used to show a user from the database
pub async fn show<R: UserRepository>(
    repository: web::Data<R>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(repository
        .get(id.into_inner())
        .await
        .map(|user| HttpResponse::Ok().json(SafeUser::from(user)))?)
}

/// This function is used to update partial content of a user from the database
pub async fn update<R: UserRepository>(repository: web::Data<R>, id: web::Path<i32>, updatable_user: web::Json<UpdatableUser>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json("Update action perfomed on the user"))
}

/// This function is used to update a user from the database
pub async fn replace<R: UserRepository>(repository: web::Data<R>, id: web::Path<i32>, new_user: web::Json<NewUser>) -> Result<HttpResponse, Error> {
    let user = User::from(NewUserWithId {
        id: id.clone(),
        user: new_user.into_inner(),
    });

    Ok(repository
        .update(id.into_inner(), &user)
        .await
        .map(|_| HttpResponse::Ok().json("Update action perfomed on the user"))?)
}

/// This function is used to delete a user from the database
pub async fn destroy<R: UserRepository>(
    repository: web::Data<R>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(repository
        .delete(id.into_inner())
        .await
        .map(|_| HttpResponse::Ok().json("Delete action perfomed on the user"))?)
}
