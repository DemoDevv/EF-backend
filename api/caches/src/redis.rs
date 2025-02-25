use std::sync::Arc;

use api_configs::config::Config;

extern crate redis;

// type initialization
pub type RedisRepositoryResult<T> = Result<T, api_errors::ServiceError>;
pub type RedisClient = Arc<redis::Client>;

// public function to get a redis client
pub fn get_redis_client(config: &Config) -> RedisClient {
    Arc::new(redis::Client::open(config.redis_info.get_url()).unwrap())
}

#[async_trait::async_trait]
pub trait RedisRepository: Clone + Send + Sync + 'static {
    async fn ping(&self) -> RedisRepositoryResult<Option<String>>;
    async fn get(&self, key: &str) -> RedisRepositoryResult<Option<String>>;
    async fn set(&self, key: &str, value: &str) -> RedisRepositoryResult<()>;
    async fn hset_multiple(
        &self,
        key: &str,
        fields: Vec<(String, String)>,
    ) -> RedisRepositoryResult<()>;
    async fn hget_multiple(
        &self,
        key: &str,
        fields: Vec<String>,
    ) -> RedisRepositoryResult<Vec<Option<String>>>;
    async fn hget(&self, key: &str, field: &str) -> RedisRepositoryResult<Option<String>>;
    async fn ttl(&self, key: &str) -> RedisRepositoryResult<i64>;
    async fn update(&self, key: &str, value: &str) -> RedisRepositoryResult<()>;
    async fn update_ttl(&self, key: &str, value: &str, ttl: i64) -> RedisRepositoryResult<()>;
    async fn delete(&self, key: &str) -> RedisRepositoryResult<()>;
}

#[async_trait::async_trait]
impl RedisRepository for RedisClient {
    async fn ping(&self) -> RedisRepositoryResult<Option<String>> {
        let mut con = self.get_multiplexed_async_connection().await?;
        redis::cmd("PING")
            .query_async(&mut con)
            .await
            .map_err(|e| e.into())
    }

    async fn get(&self, key: &str) -> RedisRepositoryResult<Option<String>> {
        let mut con = self.get_multiplexed_async_connection().await?;
        redis::cmd("GET")
            .arg(key)
            .query_async(&mut con)
            .await
            .map_err(|e| e.into())
    }

    async fn set(&self, key: &str, value: &str) -> RedisRepositoryResult<()> {
        let mut con = self.get_multiplexed_async_connection().await?;
        redis::cmd("SET")
            .arg(key)
            .arg(value)
            .query_async(&mut con)
            .await
            .map_err(|e| e.into())
    }

    async fn hset_multiple(
        &self,
        key: &str,
        fields: Vec<(String, String)>,
    ) -> RedisRepositoryResult<()> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        redis::cmd("HMSET")
            .arg(key)
            .arg(fields)
            .query_async(&mut conn)
            .await
            .map_err(|e| e.into())
    }

    async fn hget_multiple(
        &self,
        key: &str,
        fields: Vec<String>,
    ) -> RedisRepositoryResult<Vec<Option<String>>> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        redis::cmd("HMGET")
            .arg(key)
            .arg(fields)
            .query_async(&mut conn)
            .await
            .map_err(|e| e.into())
    }

    async fn hget(&self, key: &str, field: &str) -> RedisRepositoryResult<Option<String>> {
        let mut con = self.get_multiplexed_async_connection().await?;
        redis::cmd("HGET")
            .arg(key)
            .arg(field)
            .query_async(&mut con)
            .await
            .map_err(|e| e.into())
    }

    async fn ttl(&self, key: &str) -> RedisRepositoryResult<i64> {
        let mut con = self.get_multiplexed_async_connection().await?;
        redis::cmd("TTL")
            .arg(key)
            .query_async(&mut con)
            .await
            .map_err(|e| e.into())
    }

    async fn update(&self, key: &str, value: &str) -> RedisRepositoryResult<()> {
        let mut con = self.get_multiplexed_async_connection().await?;
        redis::cmd("SET")
            .arg(key)
            .arg(value)
            .query_async(&mut con)
            .await
            .map_err(|e| e.into())
    }

    async fn update_ttl(&self, key: &str, value: &str, ttl: i64) -> RedisRepositoryResult<()> {
        let mut con = self.get_multiplexed_async_connection().await?;
        redis::cmd("SET")
            .arg(key)
            .arg(value)
            .arg("EX")
            .arg(ttl)
            .query_async(&mut con)
            .await
            .map_err(|e| e.into())
    }

    async fn delete(&self, key: &str) -> RedisRepositoryResult<()> {
        let mut con = self.get_multiplexed_async_connection().await?;
        redis::cmd("DEL")
            .arg(key)
            .query_async(&mut con)
            .await
            .map_err(|e| e.into())
    }
}

mod tests {
    #[allow(unused_imports)] // bug pas important avec l'éditeur
    use super::*;
    use once_cell::sync::Lazy;

    #[allow(dead_code)] // bug pas important avec l'éditeur
    const CONFIG: Lazy<api_configs::config::Config> =
        Lazy::new(|| api_configs::config::Config::init());
    #[allow(dead_code)] // bug pas important avec l'éditeur
    const CLIENT: Lazy<RedisClient> = Lazy::new(|| get_redis_client(&CONFIG.clone()));

    #[actix_rt::test]
    async fn test_redis_ping() {
        let result = CLIENT.ping().await;
        assert_eq!(result, Ok(Some("PONG".to_string())));
    }

    #[actix_rt::test]
    async fn test_redis_set_get() {
        let key = "test_set";
        let value = "value";
        CLIENT.set(key, value).await.unwrap();
        let result = CLIENT.get(key).await.unwrap();
        assert_eq!(result, Some(value.to_string()));
    }

    #[actix_rt::test]
    async fn test_redis_no_ttl() {
        let key = "test_ttl";
        let value = "value";
        CLIENT.set(key, value).await.unwrap();
        let result = CLIENT.ttl(key).await.unwrap();
        assert_eq!(result, -1);
    }

    #[actix_rt::test]
    async fn test_redis_10_sec_ttl() {
        let key = "test_ttl_10_sec";
        let value = "value";
        CLIENT.update_ttl(key, value, 10).await.unwrap();
        let result = CLIENT.ttl(key).await.unwrap();
        assert_eq!(result, 10);
    }

    #[actix_rt::test]
    async fn test_redis_update() {
        let key = "test_update";
        let value = "value";
        CLIENT.set(key, value).await.unwrap();
        let new_value = "new_value";
        CLIENT.update(key, new_value).await.unwrap();
        let result = CLIENT.get(key).await.unwrap();
        assert_eq!(result, Some(new_value.to_string()));
    }

    #[actix_rt::test]
    async fn test_redis_delete() {
        let key = "test_delete";
        CLIENT.set(key, "value").await.unwrap();
        CLIENT.delete(key).await.unwrap();
        let result = CLIENT.get(key).await.unwrap();
        assert_eq!(result, None);
    }
}
