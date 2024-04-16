use serde::{Deserialize, Serialize};

pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub password: String,
    pub role: String,
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
