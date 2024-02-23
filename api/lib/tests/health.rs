use actix_web::{http::StatusCode, web, App};
use api_lib::handlers::health;

#[actix_web::test]
async fn test_health_get() {
    dotenv::dotenv().ok(); // ne pas oublier de charger les variables d'environnement
    let config = shared::config::Config::init();

    let app = App::new().app_data(web::Data::new(config)).configure(health::service);
    let app = actix_web::test::init_service(app).await;

    let req = actix_web::test::TestRequest::get().uri("/status").to_request();
    let resp = actix_web::test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
}
