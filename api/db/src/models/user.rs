use diesel::prelude::*;
use serde::Serialize;

use shared::types::user::SafeUser;

use crate::schema::users;

#[derive(Queryable, Selectable, Serialize, AsChangeset, Identifiable, Clone)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub password: String,
    pub role: String,
}

impl From<User> for SafeUser {
    fn from(user: User) -> Self {
        SafeUser {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            created_at: user.created_at,
            role: user.role,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct InsertableUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub created_at: chrono::NaiveDateTime,
    pub password: &'a str,
    pub role: &'a str,
}
