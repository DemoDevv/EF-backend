#[derive(Debug, PartialEq)]
pub enum RedisRepositoryError {
    NotFound,
    ParseIntError(std::num::ParseIntError),
    RedisError(redis::RedisError),
}

impl std::fmt::Display for RedisRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RedisRepositoryError::NotFound => write!(f, "Not Found"),
            RedisRepositoryError::RedisError(err) => write!(f, "Redis Error: {}", err),
            RedisRepositoryError::ParseIntError(err) => write!(f, "Parse Int Error: {}", err),
        }
    }
}

impl std::error::Error for RedisRepositoryError {}

impl From<redis::RedisError> for RedisRepositoryError {
    fn from(err: redis::RedisError) -> Self {
        RedisRepositoryError::RedisError(err)
    }
}

impl From<std::num::ParseIntError> for RedisRepositoryError {
    fn from(err: std::num::ParseIntError) -> Self {
        RedisRepositoryError::ParseIntError(err)
    }
}

#[derive(Debug, PartialEq)]
pub enum RateLimitError {
    RateLimitExceeded,
    NotFound,
    ParseIntError(std::num::ParseIntError),
    RedisRepositoryError(RedisRepositoryError),
}

impl std::fmt::Display for RateLimitError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RateLimitError::RateLimitExceeded => write!(f, "Rate Limit Exceeded"),
            RateLimitError::NotFound => write!(f, "Not Found"),
            RateLimitError::ParseIntError(err) => write!(f, "Parse Int Error: {}", err),
            RateLimitError::RedisRepositoryError(err) => {
                write!(f, "Redis Repository Error: {}", err)
            }
        }
    }
}

impl std::error::Error for RateLimitError {}

impl From<std::num::ParseIntError> for RateLimitError {
    fn from(err: std::num::ParseIntError) -> Self {
        RateLimitError::ParseIntError(err)
    }
}

impl From<RedisRepositoryError> for RateLimitError {
    fn from(err: RedisRepositoryError) -> Self {
        RateLimitError::RedisRepositoryError(err)
    }
}
