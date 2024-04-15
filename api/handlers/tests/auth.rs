use actix_web::{http::StatusCode, web, App};

use api_db::models::user::User;
use api_db::repositories::users_repository::UsersRepository;
use api_handlers::auth;

use api_db::repository::{Repository, UserRepository};
use api_services::auth::types::Tokens;
use api_services::redis::RedisRepository;
use shared::types::roles::Role;
use shared::types::user::NewUser;

use once_cell::sync::Lazy;

const CONFIG: Lazy<shared::config::Config> = Lazy::new(|| shared::config::Config::init());

async fn generate_good_user(
    users_repository: &UsersRepository,
    email: &str,
    password: &str,
) -> User {
    let hash = api_services::auth::helpers::hash_password(password).unwrap();

    // Create a valid user
    users_repository
        .create(&NewUser {
            first_name: "Jhon".to_string(),
            last_name: "Doe".to_string(),
            email: email.to_string(),
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
    let redis_client = api_services::redis::get_redis_client(&CONFIG);

    let app = App::new()
        .app_data(web::Data::new(CONFIG.clone()))
        .app_data(web::Data::new(users_repository.clone()))
        .app_data(web::Data::new(redis_client.clone()))
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
    let redis_client = api_services::redis::get_redis_client(&CONFIG);

    let email = "mathieulebras@gmail.com";
    let password = "good_password";
    let valid_user = generate_good_user(&users_repository, email, password).await;

    let app = App::new()
        .app_data(web::Data::new(CONFIG.clone()))
        .app_data(web::Data::new(users_repository.clone()))
        .app_data(web::Data::new(redis_client.clone()))
        .configure(auth::service::<UsersRepository>);
    let app = actix_web::test::init_service(app).await;

    let req = actix_web::test::TestRequest::post()
        .uri("/v1/auth/login")
        .set_json(&serde_json::json!({
            "email": email,
            "password": password
        }))
        .to_request();
    let resp = actix_web::test::call_service(&app, req).await;

    users_repository.delete(valid_user.id).await.unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_register_with_email_already_exist() {
    let users_repository = UsersRepository::new(api_db::connection::establish_connection(&CONFIG));
    let redis_client = api_services::redis::get_redis_client(&CONFIG);

    let email = "mathieulebras_exist@gmail.com";
    let password = "good_password";
    let valid_user = generate_good_user(&users_repository, email, password).await;

    let app = App::new()
        .app_data(web::Data::new(CONFIG.clone()))
        .app_data(web::Data::new(users_repository.clone()))
        .app_data(web::Data::new(redis_client.clone()))
        .configure(auth::service::<UsersRepository>);
    let app = actix_web::test::init_service(app).await;

    let req = actix_web::test::TestRequest::post()
        .uri("/v1/auth/register")
        .set_json(&serde_json::json!({
            "email": email,
            "password": password
        }))
        .to_request();
    let resp = actix_web::test::call_service(&app, req).await;

    users_repository.delete(valid_user.id).await.unwrap();

    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[actix_web::test]
async fn test_register_with_email_not_already_exist() {
    let users_repository = UsersRepository::new(api_db::connection::establish_connection(&CONFIG));
    let redis_client = api_services::redis::get_redis_client(&CONFIG);

    let email = "mathieulebras_notexist@gmail.com";

    let app = App::new()
        .app_data(web::Data::new(CONFIG.clone()))
        .app_data(web::Data::new(users_repository.clone()))
        .app_data(web::Data::new(redis_client.clone()))
        .configure(auth::service::<UsersRepository>);
    let app = actix_web::test::init_service(app).await;

    let req = actix_web::test::TestRequest::post()
        .uri("/v1/auth/register")
        .set_json(&serde_json::json!({
            "email": email,
            "password": "password"
        }))
        .to_request();
    let resp = actix_web::test::call_service(&app, req).await;

    users_repository.delete_user_by_email(email).await.unwrap();

    println!("{:?}", resp.response());

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_refresh_tokens() {
    let users_repository = UsersRepository::new(api_db::connection::establish_connection(&CONFIG));
    let redis_client = api_services::redis::get_redis_client(&CONFIG);

    let email = "mathieulebras_refreshtest@gmail.com";
    let password = "good_password";

    let valid_user = generate_good_user(&users_repository, email, password).await;

    let app = App::new()
        .app_data(web::Data::new(CONFIG.clone()))
        .app_data(web::Data::new(users_repository.clone()))
        .app_data(web::Data::new(redis_client.clone()))
        .configure(auth::service::<UsersRepository>);

    let app = actix_web::test::init_service(app).await;

    // login for get the tokens
    let req = actix_web::test::TestRequest::post()
        .uri("/v1/auth/login")
        .set_json(&serde_json::json!({
            "email": email,
            "password": password
        }))
        .to_request();

    let resp = actix_web::test::call_service(&app, req).await;
    let tokens: Tokens = actix_web::test::read_body_json(resp).await;
    let refresh_token = tokens.refresh_token;

    let req = actix_web::test::TestRequest::post()
        .uri("/v1/auth/refresh")
        .set_json(&serde_json::json!({
            "email": email,
            "refresh_token": refresh_token
        }))
        .to_request();

    let resp = actix_web::test::call_service(&app, req).await;

    users_repository.delete(valid_user.id).await.unwrap();
    redis_client.delete(&refresh_token).await.unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    let new_tokens: Tokens = actix_web::test::read_body_json(resp).await;
    redis_client.delete(&new_tokens.refresh_token).await.unwrap();
}
