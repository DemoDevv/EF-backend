use std::str::FromStr;

use crate::redis::{RedisClient, RedisRepository, RedisRepositoryResult};
use api_configs::config::Config;
use api_errors::ServiceError;

#[async_trait::async_trait]
pub trait AccessRefreshTokensCache: Clone + Send + Sync + 'static {
    async fn save_refresh_token(
        &self,
        refresh_token: &str,
        user_meta_data: UserMetaData,
    ) -> RedisRepositoryResult<()>;
    async fn get_meta_data_users_by_refresh_token(
        &self,
        refresh_token: &str,
    ) -> RedisRepositoryResult<UserMetaData>;
    async fn invalidate_and_save_token(
        &self,
        last_refresh_token: &str,
        new_refresh_token: &str,
    ) -> RedisRepositoryResult<()>;
}

#[derive(Clone)]
pub struct UserMetaData {
    pub id: String,
    pub email: String,
}

impl UserMetaData {
    fn to_redis_value(&self) -> String {
        format!("{}:{}", self.id, self.email)
    }
}

impl FromStr for UserMetaData {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splited: Vec<&str> = s.split(":").collect();
        Ok(UserMetaData {
            id: splited[0].to_string(),
            email: splited[1].to_string(),
        })
    }
}

#[derive(Clone)]
pub struct AccessRefreshTokensCacheRedis {
    client: RedisClient,
    config: Config,
}

impl AccessRefreshTokensCacheRedis {
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
