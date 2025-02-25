#[derive(Debug, PartialEq)]
pub enum RedisRepositoryError {
    NotFound,
    RedisError(redis::RedisError),
}

impl std::fmt::Display for RedisRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RedisRepositoryError::NotFound => write!(f, "Not Found"),
            RedisRepositoryError::RedisError(err) => write!(f, "Redis Error: {}", err),
        }
    }
}

impl std::error::Error for RedisRepositoryError {}

impl From<redis::RedisError> for RedisRepositoryError {
    fn from(err: redis::RedisError) -> Self {
        RedisRepositoryError::RedisError(err)
    }
}
