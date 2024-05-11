use serde::{Deserialize, Serialize};

use crate::extractors::user_extractor::UserPayload;

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub password: String,
    pub role: String,
}

pub struct NewUserWithId {
    pub id: i32,
    pub user: UserPayload,
}

#[derive(Serialize, Deserialize)]
pub struct SafeUser {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub role: String,
}
