use diesel::prelude::*;

use crate::connection::Pool;
use crate::models::user::{InsertableUser, User};
use crate::schema::users;
use api_errors::ServiceError;
use api_types::user::NewUser;

use crate::repository::{Repository, RepositoryResult, UserRepository};

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
                error_type: api_errors::ServiceErrorType::DatabaseError,
            })?)
            .map_err(|_| ServiceError {
                message: Some("Error getting user".to_string()),
                error_type: api_errors::ServiceErrorType::InternalServerError,
            })
    }

    async fn get_all(&self) -> RepositoryResult<Vec<User>> {
        users::table
            .select(User::as_select())
            .load(&mut self.conn.get().map_err(|_| ServiceError {
                message: Some("Error for getting connection to the database".to_string()),
                error_type: api_errors::ServiceErrorType::DatabaseError,
            })?)
            .map_err(|_| ServiceError {
                message: Some("Error getting all users".to_string()),
                error_type: api_errors::ServiceErrorType::InternalServerError,
            })
    }

    async fn create(&self, item: &NewUser) -> RepositoryResult<User> {
        let insertable_user = InsertableUser {
            pseudo: &item.pseudo,
            first_name: item.first_name.as_deref(),
            last_name: item.last_name.as_deref(),
            email: &item.email,
            password: item.password.as_deref(),
            google_id: item.google_id.as_deref(),
        };

        diesel::insert_into(users::table)
            .values(insertable_user)
            .returning(User::as_returning())
            .get_result(&mut self.conn.get().map_err(|_| ServiceError {
                message: Some("Error for getting connection to the database".to_string()),
                error_type: api_errors::ServiceErrorType::DatabaseError,
            })?)
            .map_err(|err| ServiceError {
                message: Some(err.to_string()),
                error_type: api_errors::ServiceErrorType::InternalServerError,
            })
    }

    async fn update(&self, id: i32, item: &User) -> RepositoryResult<User> {
        diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(item)
            .returning(User::as_returning())
            .get_result(&mut self.conn.get().map_err(|_| ServiceError {
                message: Some("Error for getting connection to the database".to_string()),
                error_type: api_errors::ServiceErrorType::DatabaseError,
            })?)
            .map_err(|_| ServiceError {
                message: Some("Error updating user".to_string()),
                error_type: api_errors::ServiceErrorType::InternalServerError,
            })
    }

    async fn delete(&self, id: i32) -> RepositoryResult<usize> {
        diesel::delete(users::table.filter(users::id.eq(id)))
            .execute(&mut self.conn.get().map_err(|_| ServiceError {
                message: Some("Error for getting connection to the database".to_string()),
                error_type: api_errors::ServiceErrorType::DatabaseError,
            })?)
            .map_err(|_| ServiceError {
                message: Some("Error deleting user".to_string()),
                error_type: api_errors::ServiceErrorType::InternalServerError,
            })
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
                error_type: api_errors::ServiceErrorType::DatabaseError,
            })?)
            .map_err(|_| ServiceError {
                message: Some("Error getting user".to_string()),
                error_type: api_errors::ServiceErrorType::InternalServerError,
            })
    }

    async fn delete_user_by_email(&self, email: &str) -> RepositoryResult<usize> {
        diesel::delete(users::table.filter(users::email.eq(email)))
            .execute(&mut self.conn.get().map_err(|_| ServiceError {
                message: Some("Error for getting connection to the database".to_string()),
                error_type: api_errors::ServiceErrorType::DatabaseError,
            })?)
            .map_err(|_| ServiceError {
                message: Some("Error deleting user".to_string()),
                error_type: api_errors::ServiceErrorType::InternalServerError,
            })
    }

    async fn get_user_by_google_id(&self, google_id: &str) -> RepositoryResult<User> {
        users::table
            .filter(users::google_id.eq(google_id))
            .select(User::as_select())
            .first(&mut self.conn.get().map_err(|_| ServiceError {
                message: Some("Error for getting connection to the database".to_string()),
                error_type: api_errors::ServiceErrorType::DatabaseError,
            })?)
            .map_err(|_| ServiceError {
                message: Some("Error getting user".to_string()),
                error_type: api_errors::ServiceErrorType::InternalServerError,
            })
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;
    use once_cell::sync::Lazy;

    #[allow(dead_code)]
    const CONFIG: Lazy<api_configs::config::Config> =
        Lazy::new(|| api_configs::config::Config::init());

    #[actix_rt::test]
    async fn test_update_user() {
        let user_repository =
            UsersRepository::new(crate::connection::establish_testing_connection(&CONFIG));

        // can result in an error if the auto increment of diesel is lower than the number of users seed manually.
        // you just have to run tests more than the number of users in the seed script
        let user = user_repository
            .create(&NewUser {
                pseudo: "pseudo".to_string(),
                first_name: Some("Jhon".to_string()),
                last_name: Some("Doe".to_string()),
                email: "emaildetest@test.com".to_string(),
                password: Some("password".to_string()),
                google_id: None,
            })
            .await;

        assert!(user.is_ok());

        let mut cloned_user = user.unwrap().clone();
        cloned_user.first_name = Some("Jane".to_string());

        let updated_user = user_repository.update(cloned_user.id, &cloned_user).await;
        assert_eq!(updated_user.is_ok(), true);

        assert_eq!(updated_user.unwrap().first_name, Some("Jane".to_string()));
    }
}
