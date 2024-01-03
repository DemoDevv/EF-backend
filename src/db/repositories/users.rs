use diesel::{PgConnection, SelectableHelper, RunQueryDsl};

use crate::schema::users;
use crate::models::user::{User, NewUser};

pub struct UsersRepository {}

impl UsersRepository {
    pub fn create_new_user(
        conn: &mut PgConnection,
        email: &str,
        password: &str
    ) -> User {
        let new_user = NewUser { first_name: "Jhon", last_name: "Doe", email, created_at: chrono::Local::now().naive_local(), password, salt: "salt_test" };

        diesel::insert_into(users::table)
            .values(new_user)
            .returning(User::as_returning())
            .get_result(conn)
            .expect("Error saving new user")
    }
}