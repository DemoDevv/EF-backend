use serde::{Deserialize, Serialize};

use validator::Validate;

#[derive(Deserialize, Validate, Debug)]
pub struct InputUser {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct UserPayload {
    #[validate(length(min = 1))]
    pub pseudo: String,
    #[validate(length(min = 1))]
    pub first_name: Option<String>,
    #[validate(length(min = 1))]
    pub last_name: Option<String>,
    #[validate(email)]
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub password: Option<String>,
    pub google_id: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct UpdatableUser {
    #[validate(email)]
    pub email: Option<String>,
    pub pseudo: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct RefreshableUser {
    #[validate(email)]
    pub email: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub pseudo: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub password: Option<String>,
    pub google_id: Option<String>,
}

pub struct NewUserWithId {
    pub id: i32,
    pub user: UserPayload,
}

#[derive(Serialize, Deserialize)]
pub struct SafeUser {
    pub id: i32,
    pub pseudo: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}
