extern crate redis;

// type initialization
type RedisServiceResult<T> = Result<T, redis::RedisError>; // todo: move to serviceError
type RedisClient = redis::Client;

// public function to get a redis client
pub fn get_redis_client() -> RedisClient {
    redis::Client::open("redis://mathieulebras@127.0.0.1:6379").unwrap()
}

#[async_trait::async_trait]
pub trait RedisRepository: Clone + Send + Sync + 'static {
    async fn ping(&self) -> RedisServiceResult<Option<String>>;
    async fn get(&self, key: &str) -> RedisServiceResult<Option<String>>;
    async fn set(&self, key: &str, value: &str) -> RedisServiceResult<()>;
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

    async fn delete(&self, key: &str) -> RedisServiceResult<()> {
        let mut con: redis::aio::Connection = self.get_async_connection().await?;
        redis::cmd("DEL").arg(key).query_async(&mut con).await
    }
}

mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_redis_ping() {
        let client = get_redis_client();
        let result = client.ping().await;
        assert_eq!(result, Ok(Some("PONG".to_string())));
    }

    #[actix_rt::test]
    async fn test_redis_set_get() {
        let client = get_redis_client();
        let key = "test";
        let value = "value";
        client.set(key, value).await.unwrap();
        let result = client.get(key).await.unwrap();
        assert_eq!(result, Some(value.to_string()));
    }

    #[actix_rt::test]
    async fn test_redis_delete() {
        let client = get_redis_client();
        let key = "test";
        client.set(key, "value").await.unwrap();
        client.delete(key).await.unwrap();
        let result = client.get(key).await.unwrap();
        assert_eq!(result, None);
    }
}
