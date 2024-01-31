use diesel::{PgConnection, RunQueryDsl, SelectableHelper};

use crate::db::schema::users;
use crate::models::user::{NewUser, User};
use crate::types::roles::Role;

pub struct UsersRepository {}

impl UsersRepository {
    pub fn create_new_user(conn: &mut PgConnection, email: &str, password: &str) -> User {
        let new_user = NewUser {
            first_name: "Jhon",
            last_name: "Doe",
            email,
            created_at: chrono::Local::now().naive_local(),
            password,
            salt: "salt_test",
            role: Role::User.to_string(),
        };

        diesel::insert_into(users::table)
            .values(new_user)
            .returning(User::as_returning())
            .get_result(conn)
            .expect("Error saving new user")
    }
}
