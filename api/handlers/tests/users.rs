use actix_web::{http::StatusCode, web, App};

use api_db::repository::Repository;
use api_db::{models::user::User, repositories::users_repository::UsersRepository};
use once_cell::sync::Lazy;

use api_handlers::users;
use api_types::{roles::Role, user::NewUser};

const CONFIG: Lazy<shared::config::Config> = Lazy::new(|| shared::config::Config::init());
const TOKEN_FOR_TEST: &str = "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJtYXRoaWV1bGVicmFhYXNAZ21haWwuY29tIiwiaWF0IjoxNzExMTI0MzQxLCJleHAiOjE3NjI5NjQzNDEsInJvbGUiOiJ1c2VyIn0.OfP32SVlG0XcV5Pf-LIJt9T6j1g0cCFaUnW00k3dL1w";

const USERS_REPOSITORY: Lazy<UsersRepository> =
    Lazy::new(|| UsersRepository::new(api_db::connection::establish_connection(&CONFIG)));

async fn generate_user(users_repository: &UsersRepository) -> User {
    let hash = api_services::auth::helpers::hash_password("test").unwrap();

    // Create a valid user
    users_repository
        .create(&NewUser {
            first_name: "Jhon".to_string(),
            last_name: "Doe".to_string(),
            email: "jhondoe@gmail.com".to_string(),
            created_at: chrono::Local::now().naive_local(),
            password: hash,
            role: Role::User.to_string(),
        })
        .await
        .unwrap()
}

#[actix_web::test]
async fn test_get_all_users() {
    let app = App::new()
        .app_data(web::Data::new(CONFIG.clone()))
        .app_data(web::Data::new(USERS_REPOSITORY.clone()))
        .configure(users::service::<UsersRepository>);
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
        .app_data(web::Data::new(USERS_REPOSITORY.clone()))
        .configure(users::service::<UsersRepository>);
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
    let user = generate_user(&USERS_REPOSITORY).await;

    let app = App::new()
        .app_data(web::Data::new(CONFIG.clone()))
        .app_data(web::Data::new(USERS_REPOSITORY.clone()))
        .configure(users::service::<UsersRepository>);
    let app = actix_web::test::init_service(app).await;

    let req = actix_web::test::TestRequest::get()
        .uri(format!("/v1/users/{}", user.id).as_str())
        .append_header(("Authorization", TOKEN_FOR_TEST))
        .to_request();
    let resp = actix_web::test::call_service(&app, req).await;

    USERS_REPOSITORY.delete(user.id).await.unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_patch_one_user() {
    let user = generate_user(&USERS_REPOSITORY).await;

    let app = App::new()
        .app_data(web::Data::new(CONFIG.clone()))
        .app_data(web::Data::new(USERS_REPOSITORY.clone()))
        .configure(users::service::<UsersRepository>);
    let app = actix_web::test::init_service(app).await;

    let req = actix_web::test::TestRequest::patch()
        .uri(format!("/v1/users/{}", user.id).as_str())
        .append_header(("Authorization", TOKEN_FOR_TEST))
        .set_json(&serde_json::json!({
            "email": "cestplusjhondoe@gmail.com",
            "password": "test",
        }))
        .to_request();
    let resp = actix_web::test::call_service(&app, req).await;

    USERS_REPOSITORY.delete(user.id).await.unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    let body = actix_web::test::read_body(resp).await;
    let user: User =
        serde_json::from_slice(&body).expect("Failed to parse response body into user");

    assert_eq!(user.email, "cestplusjhondoe@gmail.com")
}

#[actix_web::test]
async fn test_replace_one_user() {
    let user = generate_user(&USERS_REPOSITORY).await;

    let app = App::new()
        .app_data(web::Data::new(CONFIG.clone()))
        .app_data(web::Data::new(USERS_REPOSITORY.clone()))
        .configure(users::service::<UsersRepository>);
    let app = actix_web::test::init_service(app).await;

    let req = actix_web::test::TestRequest::put()
        .uri(format!("/v1/users/{}", user.id).as_str())
        .append_header(("Authorization", TOKEN_FOR_TEST))
        .set_json(&serde_json::json!({
            "first_name": "jhon",
            "last_name": "Doee",
            "email": "cestplusjhondoe@gmail.com",
            "created_at": chrono::Local::now().naive_local(),
            "password": "test",
            "role": Role::User.to_string(),
        }))
        .to_request();
    let resp = actix_web::test::call_service(&app, req).await;

    USERS_REPOSITORY.delete(user.id).await.unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    let body = actix_web::test::read_body(resp).await;
    let user: User =
        serde_json::from_slice(&body).expect("Failed to parse response body into user");

    assert_eq!(user.email, "cestplusjhondoe@gmail.com")
}

#[actix_web::test]
async fn test_delete_one_user() {
    let user = generate_user(&USERS_REPOSITORY).await;

    let app = App::new()
        .app_data(web::Data::new(CONFIG.clone()))
        .app_data(web::Data::new(USERS_REPOSITORY.clone()))
        .configure(users::service::<UsersRepository>);
    let app = actix_web::test::init_service(app).await;

    let req = actix_web::test::TestRequest::delete()
        .uri(format!("/v1/users/{}", user.id).as_str())
        .append_header(("Authorization", TOKEN_FOR_TEST))
        .to_request();
    let resp = actix_web::test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
}
