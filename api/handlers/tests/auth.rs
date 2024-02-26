use actix_web::{http::StatusCode, web, App};

use api_db::models::user::User;
use api_db::repositories::users_repository::UsersRepository;
use api_handlers::auth;

use api_db::repository::{Repository, UserRepository};
use shared::types::roles::Role;
use shared::types::user::NewUser;

use once_cell::sync::Lazy;

const CONFIG: Lazy<shared::config::Config> = Lazy::new(|| shared::config::Config::init());
const GOOD_EMAIL: &str = "goodemail@test.com";
const GOOD_PASSWORD: &str = "password";

async fn generate_good_user(users_repository: &UsersRepository) -> User {
    let good_email = GOOD_EMAIL;
    let good_password = GOOD_PASSWORD;
    let hash = api_services::auth::helpers::hash_password(good_password).unwrap();

    // Create a valid user
    users_repository
        .create(&NewUser {
            first_name: "Jhon".to_string(),
            last_name: "Doe".to_string(),
            email: good_email.to_string(),
            created_at: chrono::Local::now().naive_local(),
            password: hash,
            role: Role::User.to_string(),
        })
        .await
        .unwrap()
}

#[actix_web::test]
async fn test_login_with_bad_credentials() {
    let users_repository = UsersRepository::new(api_db::connection::establish_connection(&CONFIG));

    let app = App::new()
        .app_data(web::Data::new(CONFIG.clone()))
        .app_data(web::Data::new(users_repository.clone()))
        .configure(auth::service::<UsersRepository>);
    let app = actix_web::test::init_service(app).await;

    let req = actix_web::test::TestRequest::post()
        .uri("/v1/auth/login")
        .set_json(&serde_json::json!({
            "email": "bademail@test.com",
            "password": "badpassword"
        }))
        .to_request();
    let resp = actix_web::test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn test_login_with_good_credentials() {
    let users_repository = UsersRepository::new(api_db::connection::establish_connection(&CONFIG));

    let valid_user = generate_good_user(&users_repository).await;

    let app = App::new()
        .app_data(web::Data::new(CONFIG.clone()))
        .app_data(web::Data::new(users_repository.clone()))
        .configure(auth::service::<UsersRepository>);
    let app = actix_web::test::init_service(app).await;

    let req = actix_web::test::TestRequest::post()
        .uri("/v1/auth/login")
        .set_json(&serde_json::json!({
            "email": GOOD_EMAIL,
            "password": GOOD_PASSWORD
        }))
        .to_request();
    let resp = actix_web::test::call_service(&app, req).await;

    users_repository.delete(valid_user.id).await.unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_register_with_email_already_exist() {
    let users_repository = UsersRepository::new(api_db::connection::establish_connection(&CONFIG));

    let valid_user = generate_good_user(&users_repository).await;

    let app = App::new()
        .app_data(web::Data::new(CONFIG.clone()))
        .app_data(web::Data::new(users_repository.clone()))
        .configure(auth::service::<UsersRepository>);
    let app = actix_web::test::init_service(app).await;

    let req = actix_web::test::TestRequest::post()
        .uri("/v1/auth/register")
        .set_json(&serde_json::json!({
            "email": GOOD_EMAIL,
            "password": GOOD_PASSWORD
        }))
        .to_request();
    let resp = actix_web::test::call_service(&app, req).await;

    users_repository.delete(valid_user.id).await.unwrap();

    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[actix_web::test]
async fn test_register_with_email_not_already_exist() {
    let users_repository = UsersRepository::new(api_db::connection::establish_connection(&CONFIG));

    let app = App::new()
        .app_data(web::Data::new(CONFIG.clone()))
        .app_data(web::Data::new(users_repository.clone()))
        .configure(auth::service::<UsersRepository>);
    let app = actix_web::test::init_service(app).await;

    let req = actix_web::test::TestRequest::post()
        .uri("/v1/auth/register")
        .set_json(&serde_json::json!({
            "email": GOOD_EMAIL,
            "password": GOOD_PASSWORD
        }))
        .to_request();
    let resp = actix_web::test::call_service(&app, req).await;

    users_repository
        .delete_user_by_email(GOOD_EMAIL)
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
}
