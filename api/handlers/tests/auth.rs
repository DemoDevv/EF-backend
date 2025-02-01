use std::sync::Arc;

use actix_web::{http::StatusCode, web, App};

use api_caches::{access_refresh_tokens::AccessRefreshTokensCacheRedis, redis::RedisRepository};
use api_db::repositories::users_repository::UsersRepository;
use api_handlers::auth;
use api_services::auth::{services::AuthService, types::Tokens};

mod common;

#[actix_web::test]
async fn test_login_with_bad_credentials() {
    let redis_client = api_caches::redis::get_redis_client(&common::CONFIG);

    let users_repository = Arc::new(UsersRepository::new(
        api_db::connection::establish_testing_connection(&common::CONFIG),
    ));
    let access_refresh_tokens_cache = Arc::new(AccessRefreshTokensCacheRedis::new(
        Arc::clone(&redis_client),
        common::CONFIG.clone(),
    ));

    let auth_service = AuthService::new(users_repository, access_refresh_tokens_cache);

    let app = App::new()
        .app_data(web::Data::new(common::CONFIG.clone()))
        .app_data(web::Data::new(auth_service.clone()))
        .configure(auth::service::<UsersRepository, AccessRefreshTokensCacheRedis>);
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
    let redis_client = api_caches::redis::get_redis_client(&common::CONFIG);

    let users_repository = Arc::new(UsersRepository::new(
        api_db::connection::establish_testing_connection(&common::CONFIG),
    ));
    let access_refresh_tokens_cache = Arc::new(AccessRefreshTokensCacheRedis::new(
        Arc::clone(&redis_client),
        common::CONFIG.clone(),
    ));

    let auth_service = AuthService::new(Arc::clone(&users_repository), access_refresh_tokens_cache);

    let email = "tester@test.com";
    let password = "good_password";
    common::insert_test_user(Arc::clone(&users_repository)).await;

    let app = App::new()
        .app_data(web::Data::new(common::CONFIG.clone()))
        .app_data(web::Data::new(auth_service.clone()))
        .configure(auth::service::<UsersRepository, AccessRefreshTokensCacheRedis>);
    let app = actix_web::test::init_service(app).await;

    let req = actix_web::test::TestRequest::post()
        .uri("/v1/auth/login")
        .set_json(&serde_json::json!({
            "email": email,
            "password": password
        }))
        .to_request();
    let resp = actix_web::test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_register_with_email_already_exist() {
    let redis_client = api_caches::redis::get_redis_client(&common::CONFIG);

    let users_repository = Arc::new(UsersRepository::new(
        api_db::connection::establish_testing_connection(&common::CONFIG),
    ));
    let access_refresh_tokens_cache = Arc::new(AccessRefreshTokensCacheRedis::new(
        Arc::clone(&redis_client),
        common::CONFIG.clone(),
    ));

    let auth_service = AuthService::new(Arc::clone(&users_repository), access_refresh_tokens_cache);

    let email = "tester@test.com";
    let password = "good_password";
    common::insert_test_user(Arc::clone(&users_repository)).await;

    let app = App::new()
        .app_data(web::Data::new(common::CONFIG.clone()))
        .app_data(web::Data::new(auth_service.clone()))
        .configure(auth::service::<UsersRepository, AccessRefreshTokensCacheRedis>);
    let app = actix_web::test::init_service(app).await;

    let req = actix_web::test::TestRequest::post()
        .uri("/v1/auth/register")
        .set_json(&serde_json::json!({
            "email": email,
            "password": password
        }))
        .to_request();
    let resp = actix_web::test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn test_register_with_email_not_already_exist() {
    let redis_client = api_caches::redis::get_redis_client(&common::CONFIG);

    let users_repository = Arc::new(UsersRepository::new(
        api_db::connection::establish_testing_connection(&common::CONFIG),
    ));
    let access_refresh_tokens_cache = Arc::new(AccessRefreshTokensCacheRedis::new(
        Arc::clone(&redis_client),
        common::CONFIG.clone(),
    ));

    let auth_service = AuthService::new(users_repository, access_refresh_tokens_cache);

    let email = "mathieulebras_notexist@gmail.com";

    let app = App::new()
        .app_data(web::Data::new(common::CONFIG.clone()))
        .app_data(web::Data::new(auth_service.clone()))
        .configure(auth::service::<UsersRepository, AccessRefreshTokensCacheRedis>);
    let app = actix_web::test::init_service(app).await;

    let req = actix_web::test::TestRequest::post()
        .uri("/v1/auth/register")
        .set_json(&serde_json::json!({
            "email": email,
            "password": "password"
        }))
        .to_request();
    let resp = actix_web::test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_refresh_tokens() {
    let redis_client = api_caches::redis::get_redis_client(&common::CONFIG);

    let users_repository = Arc::new(UsersRepository::new(
        api_db::connection::establish_testing_connection(&common::CONFIG),
    ));
    let access_refresh_tokens_cache = Arc::new(AccessRefreshTokensCacheRedis::new(
        Arc::clone(&redis_client),
        common::CONFIG.clone(),
    ));

    let auth_service = AuthService::new(Arc::clone(&users_repository), access_refresh_tokens_cache);

    let email = "tester@test.com";
    let password = "good_password";

    common::insert_test_user(Arc::clone(&users_repository)).await;

    let app = App::new()
        .app_data(web::Data::new(common::CONFIG.clone()))
        .app_data(web::Data::new(auth_service.clone()))
        .configure(auth::service::<UsersRepository, AccessRefreshTokensCacheRedis>);

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

    redis_client.delete(&refresh_token).await.unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    let new_tokens: Tokens = actix_web::test::read_body_json(resp).await;
    redis_client
        .delete(&new_tokens.refresh_token)
        .await
        .unwrap();
}
