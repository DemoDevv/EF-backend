use actix_web::{http::StatusCode, web, App};

use once_cell::sync::Lazy;

use api_handlers::users;

const CONFIG: Lazy<shared::config::Config> = Lazy::new(|| shared::config::Config::init());
const TOKEN_FOR_TEST: &str = "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJtYXRoaWV1bGVicmFhYXNAZ21haWwuY29tIiwiaWF0IjoxNzExMTI0MzQxLCJleHAiOjE3NjI5NjQzNDEsInJvbGUiOiJ1c2VyIn0.OfP32SVlG0XcV5Pf-LIJt9T6j1g0cCFaUnW00k3dL1w";

#[actix_web::test]
async fn test_get_all_users() {
    let app = App::new()
        .app_data(web::Data::new(CONFIG.clone()))
        .configure(users::service);
    let app = actix_web::test::init_service(app).await;

    let req = actix_web::test::TestRequest::get()
        .uri("/v1/users")
        .append_header(("Authorization", TOKEN_FOR_TEST))
        .to_request();
    let resp = actix_web::test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_insert_new_user() {
    let app = App::new()
        .app_data(web::Data::new(CONFIG.clone()))
        .configure(users::service);
    let app = actix_web::test::init_service(app).await;

    let req = actix_web::test::TestRequest::post()
        .uri("/v1/users")
        .append_header(("Authorization", TOKEN_FOR_TEST))
        .to_request();
    let resp = actix_web::test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_get_one_user() {
    let app = App::new()
        .app_data(web::Data::new(CONFIG.clone()))
        .configure(users::service);
    let app = actix_web::test::init_service(app).await;

    let req = actix_web::test::TestRequest::get()
        .uri("/v1/users/1")
        .append_header(("Authorization", TOKEN_FOR_TEST))
        .to_request();
    let resp = actix_web::test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_patch_one_user() {
    let app = App::new()
        .app_data(web::Data::new(CONFIG.clone()))
        .configure(users::service);
    let app = actix_web::test::init_service(app).await;

    let req = actix_web::test::TestRequest::patch()
        .uri("/v1/users/1")
        .append_header(("Authorization", TOKEN_FOR_TEST))
        .to_request();
    let resp = actix_web::test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_delete_one_user() {
    let app = App::new()
        .app_data(web::Data::new(CONFIG.clone()))
        .configure(users::service);
    let app = actix_web::test::init_service(app).await;

    let req = actix_web::test::TestRequest::delete()
        .uri("/v1/users/1")
        .append_header(("Authorization", TOKEN_FOR_TEST))
        .to_request();
    let resp = actix_web::test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
}
