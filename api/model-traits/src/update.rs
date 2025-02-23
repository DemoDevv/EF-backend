pub type UpdateResult<T> = Result<T, api_errors::ServiceError>;

/// This trait is used to update a model. You can use the proc-macro `#[derive(Updatable)]` to automatically implement this trait for your model.
pub trait Updatable<T, N>
where
    N: Clone,
{
    fn perform_update(&self, updatable_data: T) -> UpdateResult<N>;
}
