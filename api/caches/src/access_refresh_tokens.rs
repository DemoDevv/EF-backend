use std::str::FromStr;

use crate::redis::{RedisClient, RedisRepository, RedisRepositoryResult};
use api_configs::config::Config;
use api_errors::ServiceError;

/// Trait for managing refresh tokens in a Redis-based cache.
#[async_trait::async_trait]
pub trait AccessRefreshTokensCache: Clone + Send + Sync + 'static {
    /// Saves a refresh token with associated user metadata to Redis.
    ///
    /// # Arguments
    /// * `refresh_token` - The refresh token string.
    /// * `user_meta_data` - Metadata about the user, such as user ID and email.
    ///
    /// # Returns
    /// A `RedisRepositoryResult` indicating the success or failure of the operation.
    async fn save_refresh_token(
        &self,
        refresh_token: &str,
        user_meta_data: UserMetaData,
    ) -> RedisRepositoryResult<()>;

    /// Retrieves user metadata associated with a refresh token from Redis.
    ///
    /// # Arguments
    /// * `refresh_token` - The refresh token string to query.
    ///
    /// # Returns
    /// A `RedisRepositoryResult` containing the `UserMetaData` associated with the refresh token.
    async fn get_meta_data_users_by_refresh_token(
        &self,
        refresh_token: &str,
    ) -> RedisRepositoryResult<UserMetaData>;

    /// Invalidates the old refresh token and saves a new refresh token in Redis.
    ///
    /// # Arguments
    /// * `last_refresh_token` - The old refresh token to invalidate.
    /// * `new_refresh_token` - The new refresh token to store.
    ///
    /// # Returns
    /// A `RedisRepositoryResult` indicating the success or failure of the operation.
    async fn invalidate_and_save_token(
        &self,
        last_refresh_token: &str,
        new_refresh_token: &str,
    ) -> RedisRepositoryResult<()>;
}

/// Structure representing user metadata (ID and email) associated with a refresh token.
#[derive(Clone)]
pub struct UserMetaData {
    /// The unique identifier of the user.
    pub id: String,
    /// The email of the user.
    pub email: String,
}

impl UserMetaData {
    /// Converts `UserMetaData` into a Redis-compatible value format.
    ///
    /// # Returns
    /// A string formatted as "id:email" to be stored in Redis.
    fn to_redis_value(&self) -> String {
        format!("{}:{}", self.id, self.email)
    }
}

impl FromStr for UserMetaData {
    type Err = ();

    /// Parses a string in the format "id:email" into a `UserMetaData` structure.
    ///
    /// # Arguments
    /// * `s` - The string to parse, expected to be in the format "id:email".
    ///
    /// # Returns
    /// A `UserMetaData` instance parsed from the input string.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splited: Vec<&str> = s.split(":").collect();
        Ok(UserMetaData {
            id: splited[0].to_string(),
            email: splited[1].to_string(),
        })
    }
}

/// Redis-based implementation of the `AccessRefreshTokensCache` trait.
#[derive(Clone)]
pub struct AccessRefreshTokensCacheRedis {
    /// Redis client instance.
    client: RedisClient,
    /// Configuration settings.
    config: Config,
}

impl AccessRefreshTokensCacheRedis {
    /// Creates a new instance of `AccessRefreshTokensCacheRedis`.
    ///
    /// # Arguments
    /// * `client` - The Redis client instance to use.
    /// * `config` - Configuration settings.
    ///
    /// # Returns
    /// A new `AccessRefreshTokensCacheRedis` instance.
    pub fn new(client: RedisClient, config: Config) -> Self {
        AccessRefreshTokensCacheRedis { client, config }
    }
}

#[async_trait::async_trait]
impl AccessRefreshTokensCache for AccessRefreshTokensCacheRedis {
    async fn save_refresh_token(
        &self,
        refresh_token: &str,
        user_meta_data: UserMetaData,
    ) -> RedisRepositoryResult<()> {
        self.client
            .update_ttl(
                refresh_token,
                &user_meta_data.to_redis_value(),
                self.config.refresh_token_ttl,
            )
            .await
    }

    async fn get_meta_data_users_by_refresh_token(
        &self,
        refresh_token: &str,
    ) -> RedisRepositoryResult<UserMetaData> {
        let user_meta_data = self.client.get(refresh_token).await?;

        if user_meta_data.is_none() {
            return Err(ServiceError {
                message: Some("We can't get the user meta data".to_string()),
                error_type: api_errors::ServiceErrorType::UnAuthorized,
            });
        }

        Ok(UserMetaData::from_str(&user_meta_data.unwrap()).unwrap())
    }

    async fn invalidate_and_save_token(
        &self,
        last_refresh_token: &str,
        new_refresh_token: &str,
    ) -> RedisRepositoryResult<()> {
        let user_meta_data = self
            .get_meta_data_users_by_refresh_token(last_refresh_token)
            .await?;

        self.client.delete(last_refresh_token).await?;
        self.save_refresh_token(new_refresh_token, user_meta_data)
            .await
    }
}
