use crate::models::user::User;
use api_types::user::NewUser;

pub type RepositoryResult<T> = Result<T, api_errors::ServiceError>;

#[async_trait::async_trait]
pub trait Repository<T, N>: Clone + Send + Sync + 'static {
    // methods global to all repositories
    async fn get(&self, id: i32) -> RepositoryResult<T>;
    async fn get_all(&self) -> RepositoryResult<Vec<T>>;
    async fn create(&self, item: &N) -> RepositoryResult<T>;
    async fn update(&self, id: i32, item: &T) -> RepositoryResult<T>;
    async fn delete(&self, id: i32) -> RepositoryResult<usize>;
}

#[async_trait::async_trait]
pub trait UserRepository: Clone + Send + Sync + 'static + Repository<User, NewUser> {
    // methods specific to the users repository
    async fn get_user_by_email(&self, email: &str) -> RepositoryResult<User>;
    async fn delete_user_by_email(&self, email: &str) -> RepositoryResult<usize>;
    async fn get_user_by_google_id(&self, google_id: &str) -> RepositoryResult<User>;
}
