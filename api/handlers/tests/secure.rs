use actix_web::{http::StatusCode, web, App};
use api_handlers::secure;

mod common;

#[actix_web::test]
async fn test_secure_jwt_with_good_jwt() {
    let app = App::new()
        .app_data(web::Data::new(common::CONFIG.clone()))
        .configure(secure::service);
    let app = actix_web::test::init_service(app).await;

    let req = actix_web::test::TestRequest::get()
        .uri("/v1/secure/test-jwt")
        .append_header(("Authorization", common::TOKEN_FOR_TEST))
        .to_request();
    let resp = actix_web::test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_secure_jwt_with_bad_jwt() {
    let app = App::new()
        .app_data(web::Data::new(common::CONFIG.clone()))
        .configure(secure::service);
    let app = actix_web::test::init_service(app).await;

    let req = actix_web::test::TestRequest::get()
        .uri("/v1/secure/test-jwt")
        .append_header(("Authorization", common::BAD_TOKEN_FOR_TEST))
        .to_request();
    let resp = actix_web::test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}
