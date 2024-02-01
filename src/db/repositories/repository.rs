use crate::errors;

pub type RepositoryResult<T> = Result<T, errors::ServiceError>;

#[async_trait::async_trait]
pub trait Repository<T>: Send + Sync + 'static {
    async fn get(&self, id: i32) -> RepositoryResult<T>;
    async fn get_all(&self) -> RepositoryResult<Vec<T>>;
    async fn create(&self, item: &T) -> RepositoryResult<T>;
    async fn update(&self, id: i32, item: &T) -> RepositoryResult<T>;
    async fn delete(&self, id: i32) -> RepositoryResult<i32>;
}