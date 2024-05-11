use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use shared::{
    extractors::user_extractor::UpdatableUser,
    types::user::{NewUserWithId, SafeUser},
};

use crate::{
    schema::users,
    update::{Updatable, UpdateResult},
};

#[derive(Queryable, Selectable, Serialize, Deserialize, AsChangeset, Identifiable, Clone)]
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

impl Updatable<UpdatableUser, User> for User {
    fn perform_convert(&self, updatable_user: UpdatableUser) -> UpdateResult<User> {
        let user = self.clone();
        let updated_user = User {
            email: updatable_user.email.unwrap_or(user.email),
            password: updatable_user.password.unwrap_or(user.password),
            ..user
        };
        Ok(updated_user)
    }
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

impl From<NewUserWithId> for User {
    fn from(user: NewUserWithId) -> Self {
        User {
            id: user.id,
            first_name: user.user.first_name,
            last_name: user.user.last_name,
            email: user.user.email,
            created_at: user.user.created_at,
            password: user.user.password,
            role: user.user.role,
        }
    }
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = users)]
pub struct InsertableUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub created_at: chrono::NaiveDateTime,
    pub password: &'a str,
    pub role: &'a str,
}
