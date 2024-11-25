use std::sync::Arc;

use api_db::{
    connection::Pool, repositories::users_repository::UsersRepository, repository::Repository,
};
use api_errors::ServiceError;
use api_types::user::SafeUser;

#[derive(Clone)]
pub struct UsersService {
    user_repository: UsersRepository,
}

impl UsersService {
    pub fn new(conn: Pool) -> Self {
        Self {
            user_repository: UsersRepository::new(Arc::clone(&conn)),
        }
    }

    pub async fn get_safe_user(&self, id: i32) -> Result<SafeUser, ServiceError> {
        self.user_repository
            .get(id)
            .await
            .map(|user| SafeUser::from(user))
    }

    pub async fn destroy_user(&self, id_user: i32) -> Result<(), ServiceError> {
        self.user_repository.delete(id_user).await?;

        Ok(())
    }
}
