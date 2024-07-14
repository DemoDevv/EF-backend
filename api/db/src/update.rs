pub type UpdateResult<T> = Result<T, api_errors::ServiceError>;

pub trait Updatable<T, N> {
    fn perform_convert(&self, updatable_data: T) -> UpdateResult<N>;
}
