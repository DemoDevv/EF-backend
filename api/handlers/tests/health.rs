use actix_web::{http::StatusCode, web, App};
use api_handlers::health;

mod common;

#[actix_web::test]
async fn test_health_get() {
    let app = App::new()
        .app_data(web::Data::new(common::CONFIG.clone()))
        .configure(health::service);
    let app = actix_web::test::init_service(app).await;

    let req = actix_web::test::TestRequest::get()
        .uri("/status")
        .to_request();
    let resp = actix_web::test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_health_get_without_config() {
    let app = App::new().configure(health::service);
    let app = actix_web::test::init_service(app).await;

    let req = actix_web::test::TestRequest::get()
        .uri("/status")
        .to_request();
    let resp = actix_web::test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
