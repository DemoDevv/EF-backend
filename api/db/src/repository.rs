use shared::{errors, types::user::NewUser};
use crate::models::user::User;

pub type RepositoryResult<T> = Result<T, errors::ServiceError>;

#[async_trait::async_trait]
pub trait Repository<T, N>: Clone + Send + Sync + 'static {
    // methods global to all repositories
    async fn get(&self, id: i32) -> RepositoryResult<T>;
    async fn get_all(&self) -> RepositoryResult<Vec<T>>;
    async fn create(&self, item: &N) -> RepositoryResult<T>;
    async fn update(&self, id: i32, item: &T) -> RepositoryResult<T>;
    async fn delete(&self, id: i32) -> RepositoryResult<i32>;
}

#[async_trait::async_trait]
pub trait UserRepository: Clone + Send + Sync + 'static + Repository<User, NewUser> {
    // methods specific to the users repository
    async fn get_user_by_email(&self, email: &str) -> RepositoryResult<User>;
}