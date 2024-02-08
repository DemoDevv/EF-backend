use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
use diesel::{ExpressionMethods, RunQueryDsl, SelectableHelper};

use crate::db::connection::Pool;
use crate::db::schema::users;
use crate::errors::ServiceError;
use crate::models::user::{InsertableUser, User};
use crate::types::user::NewUser;

use super::repository::{Repository, RepositoryResult, UserRepository};

#[derive(Clone)]
pub struct UsersRepository {
    conn: Pool,
}

impl UsersRepository {
    pub fn new(conn: Pool) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl Repository<User, NewUser> for UsersRepository {
    async fn get(&self, id: i32) -> RepositoryResult<User> {
        users::table
            .filter(users::id.eq(id))
            .select(User::as_select())
            .first(&mut self.conn.get().map_err(|_| ServiceError {
                message: Some("Error for getting connection to the database".to_string()),
                error_type: crate::errors::ServiceErrorType::DatabaseError,
            })?)
            .map_err(|_| ServiceError {
                message: Some("Error getting user".to_string()),
                error_type: crate::errors::ServiceErrorType::InternalServerError,
            })
    }

    async fn get_all(&self) -> RepositoryResult<Vec<User>> {
        users::table
            .select(User::as_select())
            .load(&mut self.conn.get().map_err(|_| ServiceError {
                message: Some("Error for getting connection to the database".to_string()),
                error_type: crate::errors::ServiceErrorType::DatabaseError,
            })?)
            .map_err(|_| ServiceError {
                message: Some("Error getting all users".to_string()),
                error_type: crate::errors::ServiceErrorType::InternalServerError,
            })
    }

    async fn create(&self, item: &NewUser) -> RepositoryResult<User> {
        let insertable_user = InsertableUser {
            first_name: &item.first_name,
            last_name: &item.last_name,
            email: &item.email,
            created_at: chrono::Local::now().naive_local(),
            password: &item.password,
            role: &item.role,
        };

        diesel::insert_into(users::table)
            .values(insertable_user)
            .returning(User::as_returning())
            .get_result(&mut self.conn.get().map_err(|_| ServiceError {
                message: Some("Error for getting connection to the database".to_string()),
                error_type: crate::errors::ServiceErrorType::DatabaseError,
            })?)
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

#[async_trait::async_trait]
impl UserRepository for UsersRepository {
    async fn get_user_by_email(&self, email: &str) -> RepositoryResult<User> {
        users::table
            .filter(users::email.eq(email))
            .select(User::as_select())
            .first(&mut self.conn.get().map_err(|_| ServiceError {
                message: Some("Error for getting connection to the database".to_string()),
                error_type: crate::errors::ServiceErrorType::DatabaseError,
            })?)
            .map_err(|_| ServiceError {
                message: Some("Error getting user".to_string()),
                error_type: crate::errors::ServiceErrorType::InternalServerError,
            })
    }
}
