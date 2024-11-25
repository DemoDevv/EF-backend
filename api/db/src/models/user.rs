use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use api_types::user::{NewUserWithId, SafeUser, UpdatableUser};

use crate::{
    schema::users,
    update::{Updatable, UpdateResult},
};

#[derive(Queryable, Selectable, Serialize, Deserialize, AsChangeset, Identifiable, Clone)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub pseudo: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub password: Option<String>,
    pub google_id: Option<String>,
}

impl Updatable<UpdatableUser, User> for User {
    fn perform_convert(&self, updatable_user: UpdatableUser) -> UpdateResult<User> {
        let user = self.clone();
        let updated_user = User {
            pseudo: updatable_user.pseudo.unwrap_or(user.pseudo),
            email: updatable_user.email.unwrap_or(user.email),
            ..user
        };
        Ok(updated_user)
    }
}

impl From<User> for SafeUser {
    fn from(user: User) -> Self {
        SafeUser {
            id: user.id,
            pseudo: user.pseudo,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            created_at: user.created_at,
        }
    }
}

impl From<NewUserWithId> for User {
    fn from(user: NewUserWithId) -> Self {
        User {
            id: user.id,
            pseudo: user.user.pseudo,
            first_name: user.user.first_name,
            last_name: user.user.last_name,
            email: user.user.email,
            created_at: user.user.created_at,
            password: user.user.password,
            google_id: user.user.google_id,
        }
    }
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = users)]
pub struct InsertableUser<'a> {
    pub pseudo: &'a str,
    pub first_name: Option<&'a str>,
    pub last_name: Option<&'a str>,
    pub email: &'a str,
    pub password: Option<&'a str>,
    pub google_id: Option<&'a str>,
}
