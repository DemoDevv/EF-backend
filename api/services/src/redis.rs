use shared::config::Config;

extern crate redis;

// type initialization
type RedisServiceResult<T> = Result<T, redis::RedisError>; // todo: move to serviceError
pub type RedisClient = redis::Client;

// public function to get a redis client
pub fn get_redis_client(config: &Config) -> RedisClient {
    redis::Client::open(config.redis_info.get_url()).unwrap()
}

#[async_trait::async_trait]
pub trait RedisRepository: Clone + Send + Sync + 'static {
    async fn ping(&self) -> RedisServiceResult<Option<String>>;
    async fn get(&self, key: &str) -> RedisServiceResult<Option<String>>;
    async fn set(&self, key: &str, value: &str) -> RedisServiceResult<()>;
    async fn ttl(&self, key: &str) -> RedisServiceResult<i32>;
    async fn update(&self, key: &str, value: &str) -> RedisServiceResult<()>;
    async fn update_ttl(&self, key: &str, value: &str, ttl: i32) -> RedisServiceResult<()>;
    async fn delete(&self, key: &str) -> RedisServiceResult<()>;
}

#[async_trait::async_trait]
impl RedisRepository for RedisClient {
    async fn ping(&self) -> RedisServiceResult<Option<String>> {
        let mut con: redis::aio::Connection = self.get_async_connection().await?;
        redis::cmd("PING").query_async(&mut con).await
    }

    async fn get(&self, key: &str) -> RedisServiceResult<Option<String>> {
        let mut con: redis::aio::Connection = self.get_async_connection().await?;
        redis::cmd("GET").arg(key).query_async(&mut con).await
    }

    async fn set(&self, key: &str, value: &str) -> RedisServiceResult<()> {
        let mut con: redis::aio::Connection = self.get_async_connection().await?;
        redis::cmd("SET")
            .arg(key)
            .arg(value)
            .query_async(&mut con)
            .await
    }

    async fn ttl(&self, key: &str) -> RedisServiceResult<i32> {
        let mut con: redis::aio::Connection = self.get_async_connection().await?;
        redis::cmd("TTL").arg(key).query_async(&mut con).await
    }

    async fn update(&self, key: &str, value: &str) -> RedisServiceResult<()> {
        let mut con: redis::aio::Connection = self.get_async_connection().await?;
        redis::cmd("SET")
            .arg(key)
            .arg(value)
            .query_async(&mut con)
            .await
    }

    async fn update_ttl(&self, key: &str, value: &str, ttl: i32) -> RedisServiceResult<()> {
        let mut con: redis::aio::Connection = self.get_async_connection().await?;
        redis::cmd("SET")
            .arg(key)
            .arg(value)
            .arg("EX")
            .arg(ttl)
            .query_async(&mut con)
            .await
    }

    async fn delete(&self, key: &str) -> RedisServiceResult<()> {
        let mut con: redis::aio::Connection = self.get_async_connection().await?;
        redis::cmd("DEL").arg(key).query_async(&mut con).await
    }
}

mod tests {
    #[allow(unused_imports)] // bug pas important avec l'éditeur
    use super::*;
    use once_cell::sync::Lazy;

    #[allow(dead_code)] // bug pas important avec l'éditeur
    const CONFIG: Lazy<shared::config::Config> = Lazy::new(|| shared::config::Config::init());
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
