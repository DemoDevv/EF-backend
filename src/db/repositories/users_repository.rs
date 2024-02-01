use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
use diesel::{ExpressionMethods, OptionalExtension, PgConnection, RunQueryDsl, SelectableHelper};

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

    pub fn find_user_by_email(conn: &mut PgConnection, email: &str) -> Option<User> {
        users::table
            .filter(users::email.eq(email))
            .select(User::as_select())
            .first(conn)
            .optional()
            .expect("Error finding user by email")
    }
}
