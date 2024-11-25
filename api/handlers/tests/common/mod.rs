use std::sync::Arc;

use api_caches::redis::RedisClient;
use api_db::repository::Repository;
use api_db::{models::user::User, repositories::users_repository::UsersRepository};
use api_types::user::NewUser;
use once_cell::sync::Lazy;

#[allow(dead_code)]
pub const TOKEN_FOR_TEST: &str = "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjUxLCJpYXQiOjE3MjYwNjU5MzksImV4cCI6MjMyNjA2NTkzOX0.mu1wR9ElLaQBdr0pSkwlh5IBuv3_NV-_FQuePkVbKfc";
#[allow(dead_code)]
pub const BAD_TOKEN_FOR_TEST: &str = "Bearer eyJ0eXAiOiJKV1QiLCJiOiJIUzI1NiJ9.eyJzdWIiOiJtYXRoaWV1bGVicmFhYXNAZ21haWwuY29tIiwiaWF0IjoxNzExMTI0MzQxLCJleHAiOjE3NjbGUiOiJ1c2VyIn0.OfP32SVlG0XcV5Pf-LIJt9T6j1g0cCFa";

pub const CONFIG: Lazy<api_configs::config::Config> =
    Lazy::new(|| api_configs::config::Config::init());

#[allow(dead_code)]
pub const REDIS_CLIENT: Lazy<RedisClient> =
    Lazy::new(|| api_caches::redis::get_redis_client(&CONFIG));

#[allow(dead_code)]
pub const USER_SERVICE: Lazy<api_services::users::UsersService> = Lazy::new(|| {
    api_services::users::UsersService::new(api_db::connection::establish_connection(&CONFIG))
});

#[allow(dead_code)]
pub async fn insert_test_user(users_repository: Arc<UsersRepository>) -> User {
    let hash = api_services::auth::helpers::hash_password("good_password").unwrap();

    // Create a valid user
    users_repository
        .create(&NewUser {
            pseudo: "tester".to_string(),
            first_name: Some("Jhon".to_string()),
            last_name: Some("Doe".to_string()),
            email: "tester@test.com".to_string(),
            password: Some(hash),
            google_id: None,
        })
        .await
        .unwrap()
}
