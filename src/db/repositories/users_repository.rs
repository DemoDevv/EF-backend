use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
use diesel::{ExpressionMethods, OptionalExtension, PgConnection, RunQueryDsl, SelectableHelper};

use crate::db::connection::Pool;
use crate::db::schema::users;
use crate::errors::ServiceError;
use crate::models::user::{InsertableUser, User};
use crate::types::user::NewUser;

use super::repository::{Repository, RepositoryResult};

#[derive(Clone)]
pub struct UsersRepository {
    conn: Pool,
}

impl UsersRepository {
    pub fn new(conn: Pool) -> Self {
        Self { conn }
    }

    // pub fn create_new_user(conn: &mut PgConnection, email: &str, password: &str) -> User {
    //     let new_user = NewUser {
    //         first_name: "Jhon",
    //         last_name: "Doe",
    //         email,
    //         created_at: chrono::Local::now().naive_local(),
    //         password,
    //         salt: "salt_test",
    //         role: Role::User.to_string(),
    //     };

    //     diesel::insert_into(users::table)
    //         .values(new_user)
    //         .returning(User::as_returning())
    //         .get_result(conn)
    //         .expect("Error saving new user")
    // }

    pub fn find_user_by_email(conn: &mut PgConnection, email: &str) -> Option<User> {
        users::table
            .filter(users::email.eq(email))
            .select(User::as_select())
            .first(conn)
            .optional()
            .expect("Error finding user by email")
    }
}

#[async_trait::async_trait]
impl Repository<User, NewUser> for UsersRepository {
    async fn get(&self, id: i32) -> RepositoryResult<User> {
        todo!("get user by id")
    }

    async fn get_all(&self) -> RepositoryResult<Vec<User>> {
        todo!("get all users")
    }

    async fn create(&self, item: &NewUser) -> RepositoryResult<User> {
        let insertable_user = InsertableUser {
            first_name: &item.first_name,
            last_name: &item.last_name,
            email: &item.email,
            created_at: chrono::Local::now().naive_local(),
            password: &item.password,
            salt: &item.salt,
            role: &item.role,
        };

        diesel::insert_into(users::table)
            .values(insertable_user)
            .returning(User::as_returning())
            .get_result(&mut self.conn.get().expect("couldn't get db connection from pool"))
            .map_err(|_| ServiceError {
                message: Some("Error saving new user".to_string()),
                error_type: crate::errors::ServiceErrorType::InternalServerError,
            })
    }

    async fn update(&self, id: i32, item: &User) -> RepositoryResult<User> {
        todo!("update user by id")
    }

    async fn delete(&self, id: i32) -> RepositoryResult<i32> {
        todo!("delete user by id")
    }
}
