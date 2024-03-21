use diesel::prelude::*;

use crate::connection::Pool;
use crate::models::user::{InsertableUser, User};
use crate::schema::users;
use shared::errors::ServiceError;
use shared::types::user::NewUser;

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
                error_type: shared::errors::ServiceErrorType::DatabaseError,
            })?)
            .map_err(|_| ServiceError {
                message: Some("Error getting user".to_string()),
                error_type: shared::errors::ServiceErrorType::InternalServerError,
            })
    }

    async fn get_all(&self) -> RepositoryResult<Vec<User>> {
        users::table
            .select(User::as_select())
            .load(&mut self.conn.get().map_err(|_| ServiceError {
                message: Some("Error for getting connection to the database".to_string()),
                error_type: shared::errors::ServiceErrorType::DatabaseError,
            })?)
            .map_err(|_| ServiceError {
                message: Some("Error getting all users".to_string()),
                error_type: shared::errors::ServiceErrorType::InternalServerError,
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
                error_type: shared::errors::ServiceErrorType::DatabaseError,
            })?)
            .map_err(|_| ServiceError {
                message: Some("Error saving new user".to_string()),
                error_type: shared::errors::ServiceErrorType::InternalServerError,
            })
    }

    async fn update(&self, id: i32, item: &User) -> RepositoryResult<User> {
        diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(item)
            .returning(User::as_returning())
            .get_result(&mut self.conn.get().map_err(|_| ServiceError {
                message: Some("Error for getting connection to the database".to_string()),
                error_type: shared::errors::ServiceErrorType::DatabaseError,
            })?)
            .map_err(|_| ServiceError {
                message: Some("Error updating user".to_string()),
                error_type: shared::errors::ServiceErrorType::InternalServerError,
            })
    }

    async fn delete(&self, id: i32) -> RepositoryResult<usize> {
        diesel::delete(users::table.filter(users::id.eq(id)))
            .execute(&mut self.conn.get().map_err(|_| ServiceError {
                message: Some("Error for getting connection to the database".to_string()),
                error_type: shared::errors::ServiceErrorType::DatabaseError,
            })?)
            .map_err(|_| ServiceError {
                message: Some("Error deleting user".to_string()),
                error_type: shared::errors::ServiceErrorType::InternalServerError,
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
                error_type: shared::errors::ServiceErrorType::DatabaseError,
            })?)
            .map_err(|_| ServiceError {
                message: Some("Error getting user".to_string()),
                error_type: shared::errors::ServiceErrorType::InternalServerError,
            })
    }

    async fn delete_user_by_email(&self, email: &str) -> RepositoryResult<usize> {
        diesel::delete(users::table.filter(users::email.eq(email)))
            .execute(&mut self.conn.get().map_err(|_| ServiceError {
                message: Some("Error for getting connection to the database".to_string()),
                error_type: shared::errors::ServiceErrorType::DatabaseError,
            })?)
            .map_err(|_| ServiceError {
                message: Some("Error deleting user".to_string()),
                error_type: shared::errors::ServiceErrorType::InternalServerError,
            })
    }
}

mod tests {
    #[allow(unused_imports)] // bug pas important avec l'éditeur
    use super::*;
    use once_cell::sync::Lazy;

    #[allow(dead_code)] // bug pas important avec l'éditeur
    const CONFIG: Lazy<shared::config::Config> = Lazy::new(|| shared::config::Config::init());
    #[allow(dead_code)] // bug pas important avec l'éditeur
    const USER_REPOSITORY: Lazy<UsersRepository> =
        Lazy::new(|| UsersRepository::new(crate::connection::establish_connection(&CONFIG)));

    #[allow(dead_code)] // bug pas important avec l'éditeur
    const GOOD_EMAIL: &str = "goodemailrepo@test.com";
    #[allow(dead_code)] // bug pas important avec l'éditeur
    const GOOD_PASSWORD: &str = "password";

    #[allow(dead_code)] // bug pas important avec l'éditeur
    async fn generate_good_user(users_repository: &UsersRepository) -> User {
        users_repository
            .create(&NewUser {
                first_name: "Jhon".to_string(),
                last_name: "Doe".to_string(),
                email: GOOD_EMAIL.to_string(),
                created_at: chrono::Local::now().naive_local(),
                password: GOOD_PASSWORD.to_string(),
                role: shared::types::roles::Role::User.to_string(),
            })
            .await
            .unwrap()
    }

    #[actix_rt::test]
    async fn test_update_user() {
        let user = generate_good_user(&USER_REPOSITORY).await;

        let mut cloned_user = user.clone();
        cloned_user.first_name = "Jane".to_string();

        let updated_user = USER_REPOSITORY.update(user.id, &cloned_user).await;
        assert_eq!(updated_user.is_ok(), true);

        USER_REPOSITORY.delete(user.id).await.unwrap();

        assert_eq!(updated_user.unwrap().first_name, "Jane".to_string());
    }
}
