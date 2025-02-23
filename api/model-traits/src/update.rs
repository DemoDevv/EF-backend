pub type UpdateResult<T> = Result<T, api_errors::ServiceError>;

pub trait Updatable<T, N>
where
    N: Clone,
{
    fn perform_update(&self, updatable_data: T) -> UpdateResult<N>;
}
