use actix_web::{http::StatusCode, web, App};

use api_handlers::auth;
use api_db::repositories::users_repository::UsersRepository;

use api_db::repository::Repository;
use shared::types::user::NewUser;
use shared::types::roles::Role;

#[actix_web::test]
async fn test_login_with_bad_credentials() {
    dotenv::dotenv().ok(); // ne pas oublier de charger les variables d'environnement
    let config = shared::config::Config::init();

    let users_repository = UsersRepository::new(api_db::connection::establish_connection());

    let app = App::new()
        .app_data(web::Data::new(config))
        .app_data(web::Data::new(users_repository))
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
    dotenv::dotenv().ok(); // ne pas oublier de charger les variables d'environnement
    let config = shared::config::Config::init();

    let users_repository = UsersRepository::new(api_db::connection::establish_connection());

    let good_email = "goodemail@test.com";
    let good_password = "password";
    let hash = api_services::auth::helpers::hash_password(good_password).unwrap();

    // Create a valid user
    let _ = users_repository.create(&NewUser {
        first_name: "Jhon".to_string(),
        last_name: "Doe".to_string(),
        email: good_email.to_string(),
        created_at: chrono::Local::now().naive_local(),
        password: hash,
        role: Role::User.to_string(),
    })
    .await.unwrap();

    let app = App::new()
        .app_data(web::Data::new(config))
        .app_data(web::Data::new(users_repository))
        .configure(auth::service::<UsersRepository>);
    let app = actix_web::test::init_service(app).await;

    let req = actix_web::test::TestRequest::post()
        .uri("/v1/auth/login")
        .set_json(&serde_json::json!({
            "email": good_email,
            "password": good_password
        }))
        .to_request();
    let resp = actix_web::test::call_service(&app, req).await;

    // TODO: faire la méthode delete du repo pour supprimer l'utilisateur après le test
    // users_repository.delete(id)

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_register_with_email_already_exist() {
    dotenv::dotenv().ok(); // ne pas oublier de charger les variables d'environnement
    let config = shared::config::Config::init();

    let users_repository = UsersRepository::new(api_db::connection::establish_connection());

    let app = App::new()
        .app_data(web::Data::new(config))
        .app_data(web::Data::new(users_repository))
        .configure(auth::service::<UsersRepository>);
    let app = actix_web::test::init_service(app).await;

    let req = actix_web::test::TestRequest::post()
        .uri("/v1/auth/register")
        .set_json(&serde_json::json!({
            "email": "goodemail@test.com",
            "password": "badpassword"
        }))
        .to_request();
    let resp = actix_web::test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[actix_web::test]
async fn test_register_with_email_not_already_exist() {
    dotenv::dotenv().ok(); // ne pas oublier de charger les variables d'environnement
    let config = shared::config::Config::init();

    let users_repository = UsersRepository::new(api_db::connection::establish_connection());

    let app = App::new()
        .app_data(web::Data::new(config))
        .app_data(web::Data::new(users_repository))
        .configure(auth::service::<UsersRepository>);
    let app = actix_web::test::init_service(app).await;

    let req = actix_web::test::TestRequest::post()
        .uri("/v1/auth/register")
        .set_json(&serde_json::json!({
            "email": "new@test.com",
            "password": "badpassword"
        }))
        .to_request();
    let resp = actix_web::test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
}
