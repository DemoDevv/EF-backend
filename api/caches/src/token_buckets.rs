use crate::{
    errors::RateLimitError,
    redis::{RedisClient, RedisRepository},
};
use actix_web::http::Method;
use chrono::{DateTime, Utc};

/// The refill rate in tokens per second.
const REFILL_RATE_PER_SECOND: i64 = 10;
/// The default maximum capacity of the token bucket.
const DEFAULT_CAPACITY: u64 = 100;

pub type RateLimiterResult<T> = Result<T, RateLimitError>;

/// A structure representing a token bucket for rate limiting.
#[derive(Clone)]
pub struct TokenBucket {
    /// The maximum number of tokens the bucket can hold.
    capacity: u64,
    /// The current number of available tokens.
    tokens: u64,
    /// The last time the bucket was refilled.
    last_refill_time: DateTime<Utc>,
}

impl Default for TokenBucket {
    fn default() -> Self {
        // TODO: ajouter dans le .env
        TokenBucket {
            capacity: DEFAULT_CAPACITY,
            tokens: 100,
            last_refill_time: Utc::now(),
        }
    }
}

fn _from(bucket: &TokenBucket) -> Vec<(String, String)> {
    vec![
        ("capacity".to_string(), bucket.capacity.to_string()),
        ("tokens".to_string(), bucket.tokens.to_string()),
        (
            "last_refill_time".to_string(),
            bucket.last_refill_time.to_rfc3339(),
        ),
    ]
}

/// Converts a `TokenBucket` into a vector of key-value string pairs for Redis storage.
impl From<TokenBucket> for Vec<(String, String)> {
    fn from(bucket: TokenBucket) -> Self {
        _from(&bucket)
    }
}

/// Converts a reference to `TokenBucket` into a vector of key-value string pairs for Redis storage.
impl From<&TokenBucket> for Vec<(String, String)> {
    fn from(bucket: &TokenBucket) -> Self {
        _from(bucket)
    }
}

impl TokenBucket {
    /// Creates a `TokenBucket` from cached Redis values.
    ///
    /// # Arguments
    /// * `cache` - A vector of optional strings representing Redis values.
    ///
    /// # Returns
    /// A `TokenBucket` initialized from the cache.
    fn from_cache(cache: Vec<Option<String>>) -> Self {
        TokenBucket {
            capacity: cache[0]
                .as_ref()
                .and_then(|el| el.parse().ok())
                .unwrap_or(DEFAULT_CAPACITY),
            tokens: cache[1]
                .as_ref()
                .and_then(|el| el.parse().ok())
                .unwrap_or(0),
            last_refill_time: cache[2]
                .as_ref()
                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.to_utc())
                .unwrap_or_else(Utc::now),
        }
    }

    /// Refills the token bucket based on the elapsed time since the last refill.
    fn refill(&mut self) {
        let now = Utc::now();
        let elapsed = now.signed_duration_since(self.last_refill_time);

        let tokens_to_add = (elapsed.num_seconds() * REFILL_RATE_PER_SECOND).max(0);
        self.tokens = self
            .tokens
            .saturating_add(tokens_to_add as u64)
            .min(self.capacity);
        self.last_refill_time = now;
    }
}

/// Trait for managing token buckets in a Redis-based cache.
#[async_trait::async_trait]
pub trait TokenBucketsCache: Clone + Send + Sync + 'static {
    /// Saves a token bucket to Redis.
    async fn save_bucket(&self, id: &str, bucket: &TokenBucket) -> RateLimiterResult<()>;
    /// Creates a new token bucket with default values in Redis.
    async fn create_bucket(&self, id: &str) -> RateLimiterResult<()>;
    /// Checks if a token bucket exists in Redis.
    async fn bucket_exists(&self, id: &str) -> RateLimiterResult<bool>;
    /// Refills an existing token bucket and updates Redis.
    async fn refill_bucket(&self, id: &str) -> RateLimiterResult<()>;
    /// Consumes tokens from a token bucket and updates Redis.
    async fn consume_tokens(&self, id: &str, method: &Method) -> RateLimiterResult<()>;
}

/// Redis-based implementation of `TokenBucketsCache`.
#[derive(Clone)]
pub struct TokenBucketsCacheRedis {
    /// Redis client instance.
    client: RedisClient,
    /// Prefix used for Redis keys.
    prefix: String,
}

impl TokenBucketsCacheRedis {
    /// Creates a new instance of `TokenBucketsCacheRedis`.
    ///
    /// # Arguments
    /// * `client` - The Redis client instance.
    ///
    /// # Returns
    /// A new `TokenBucketsCacheRedis` instance.
    pub fn new(client: RedisClient) -> Self {
        TokenBucketsCacheRedis {
            client,
            prefix: "ratelimit".to_string(),
        }
    }
}

#[async_trait::async_trait]
impl TokenBucketsCache for TokenBucketsCacheRedis {
    /// Saves a token bucket to Redis.
    ///
    /// # Arguments
    /// * `id` - The unique identifier for the token bucket (ip or uuid).
    /// * `bucket` - The token bucket instance to save.
    async fn save_bucket(&self, id: &str, bucket: &TokenBucket) -> RateLimiterResult<()> {
        log::info!("Saving token bucket for {}", id);
        self.client
            .hset_multiple(&format!("{}:{}", self.prefix, id), bucket.into())
            .await
            .map_err(RateLimitError::from)
    }

    /// Creates a new token bucket in Redis with default values.
    ///
    /// # Arguments
    /// * `id` - The unique identifier for the token bucket (ip or uuid).
    async fn create_bucket(&self, id: &str) -> RateLimiterResult<()> {
        log::info!("Creating token bucket for {}", id);
        self.client
            .hset_multiple(
                &format!("{}:{}", self.prefix, id),
                TokenBucket::default().into(),
            )
            .await
            .map_err(RateLimitError::from)
    }

    /// Checks if a token bucket exists in Redis.
    ///
    /// # Arguments
    /// * `id` - The unique identifier for the token bucket (ip or uuid).
    async fn bucket_exists(&self, id: &str) -> RateLimiterResult<bool> {
        self.client
            .exists(&format!("{}:{}", self.prefix, id))
            .await
            .map_err(RateLimitError::from)
    }

    /// Refills an existing token bucket and updates Redis.
    ///
    /// # Arguments
    /// * `id` - The unique identifier for the token bucket (ip or uuid).
    async fn refill_bucket(&self, id: &str) -> RateLimiterResult<()> {
        let bucket_from_cache = self
            .client
            .hget_multiple(
                &format!("{}:{}", self.prefix, id),
                vec![
                    "capacity".to_string(),
                    "tokens".to_string(),
                    "last_refill_time".to_string(),
                ],
            )
            .await?;

        if bucket_from_cache.iter().all(|v| v.is_none()) {
            return Err(RateLimitError::NotFound);
        }

        let mut bucket = TokenBucket::from_cache(bucket_from_cache);
        bucket.refill();
        log::info!("Token bucket for {} refilled", id);
        self.save_bucket(id, &bucket).await
    }

    /// Consumes tokens from a token bucket and updates Redis.
    ///
    /// # Arguments
    /// * `id` - The unique identifier for the token bucket (ip or uuid).
    /// * `method` - The HTTP method used to consume tokens.
    async fn consume_tokens(&self, id: &str, method: &Method) -> RateLimiterResult<()> {
        let tokens = self
            .client
            .hget(&format!("{}:{}", self.prefix, id), "tokens")
            .await?;

        if tokens.is_none() {
            return Err(RateLimitError::NotFound);
        }

        let tokens = tokens.unwrap().parse::<u64>()?;

        let used_tokens = match method {
            &Method::GET => 1,
            _ => 5,
        };

        let new_tokens = tokens.saturating_sub(used_tokens);

        if new_tokens == 0 {
            return Err(RateLimitError::RateLimitExceeded);
        }

        log::info!("Consuming tokens for id: {}, method: {:?}", id, method);

        self.client
            .hset(
                &format!("{}:{}", self.prefix, id),
                "tokens",
                &(tokens - used_tokens).to_string(),
            )
            .await?;

        Ok(())
    }
}

mod tests {
    use super::*;
    use once_cell::sync::Lazy;

    static CONFIG: Lazy<api_configs::config::Config> = Lazy::new(api_configs::config::Config::init);
    #[allow(dead_code)]
    static CLIENT: Lazy<RedisClient> =
        Lazy::new(|| crate::redis::get_redis_client(&CONFIG.clone()));

    #[actix_rt::test]
    async fn test_create_default_token_bucket() {
        let cache = TokenBucketsCacheRedis::new(CLIENT.clone());

        let id = "test-1";
        cache.create_bucket(id).await.unwrap();

        let bucket_from_cache = cache
            .client
            .hget_multiple(
                &format!("ratelimit:{}", id),
                vec![
                    "capacity".to_string(),
                    "tokens".to_string(),
                    "last_refill_time".to_string(),
                ],
            )
            .await
            .unwrap();

        // Vérification des valeurs par défaut
        assert_eq!(bucket_from_cache[0].as_ref().unwrap(), "100"); // Capacity
        assert_eq!(bucket_from_cache[1].as_ref().unwrap(), "100"); // Tokens
        assert!(bucket_from_cache[2].is_some()); // Last refill time should exist

        cache
            .client
            .delete(&format!("ratelimit:{}", id))
            .await
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_refill_token_bucket() {
        let cache = TokenBucketsCacheRedis::new(CLIENT.clone());

        let id = "test-2";
        cache.create_bucket(id).await.unwrap();

        let bucket_from_cache = cache
            .client
            .hget_multiple(
                &format!("ratelimit:{}", id),
                vec![
                    "capacity".to_string(),
                    "tokens".to_string(),
                    "last_refill_time".to_string(),
                ],
            )
            .await
            .unwrap();

        let mut bucket = TokenBucket::from_cache(bucket_from_cache);
        bucket.tokens = bucket.tokens - 50;
        let initial_tokens = bucket.tokens;

        // Simuler l'écoulement du temps (par exemple 5 secondes)
        std::thread::sleep(std::time::Duration::new(5, 0));

        bucket.refill(); // Refill the bucket

        // Vérifier que le nombre de tokens a été mis à jour
        assert!(bucket.tokens > initial_tokens);

        cache
            .client
            .delete(&format!("ratelimit:{}", id))
            .await
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_save_token_bucket_to_redis() {
        let cache = TokenBucketsCacheRedis::new(CLIENT.clone());

        let id = "test-3";
        let bucket = TokenBucket::default(); // Création du bucket par défaut
        cache.save_bucket(id, &bucket).await.unwrap(); // Sauvegarde dans Redis

        // Vérification que les données ont bien été sauvegardées dans Redis simulé
        let saved_bucket = cache
            .client
            .hget_multiple(
                &format!("ratelimit:{}", id),
                vec![
                    "capacity".to_string(),
                    "tokens".to_string(),
                    "last_refill_time".to_string(),
                ],
            )
            .await
            .unwrap();

        assert_eq!(saved_bucket[0].as_ref().unwrap(), "100");
        assert_eq!(saved_bucket[1].as_ref().unwrap(), "100");

        cache
            .client
            .delete(&format!("ratelimit:{}", id))
            .await
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_error_handling_redis_get() {
        let cache = TokenBucketsCacheRedis::new(CLIENT.clone());

        let id = "non-existent-id";

        // Essayons de récupérer un bucket qui n'existe pas
        let result = cache
            .client
            .hget_multiple(
                &format!("ratelimit:{}", id),
                vec![
                    "capacity".to_string(),
                    "tokens".to_string(),
                    "last_refill_time".to_string(),
                ],
            )
            .await;

        assert!(result.is_ok_and(|el| el.iter().all(|v| v.is_none())));
    }

    #[actix_rt::test]
    async fn test_error_handling_refill() {
        let cache = TokenBucketsCacheRedis::new(CLIENT.clone());

        let id = "non-existent-id-2";

        // Essayons de récupérer un bucket qui n'existe pas
        let result = cache.refill_bucket(id).await;

        assert!(result.is_err());
    }

    #[actix_rt::test]
    async fn test_from_cache() {
        let cache = TokenBucketsCacheRedis::new(CLIENT.clone());

        let id = "test-4";
        cache.create_bucket(id).await.unwrap();

        // Récupérer les données du bucket depuis Redis
        let bucket_from_cache = cache
            .client
            .hget_multiple(
                &format!("ratelimit:{}", id),
                vec![
                    "capacity".to_string(),
                    "tokens".to_string(),
                    "last_refill_time".to_string(),
                ],
            )
            .await
            .unwrap();

        // Vérifier que le `TokenBucket` est correctement créé à partir du cache
        let bucket = TokenBucket::from_cache(bucket_from_cache);
        assert_eq!(bucket.capacity, 100);
        assert_eq!(bucket.tokens, 100);

        cache
            .client
            .delete(&format!("ratelimit:{}", id))
            .await
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_short_consume_tokens() {
        let cache = TokenBucketsCacheRedis::new(CLIENT.clone());

        let id = "test-5";
        cache.create_bucket(id).await.unwrap();

        cache.consume_tokens(id, &Method::GET).await.unwrap();

        // Récupérer les données du bucket depuis Redis
        let bucket_from_cache = cache
            .client
            .hget_multiple(
                &format!("ratelimit:{}", id),
                vec![
                    "capacity".to_string(),
                    "tokens".to_string(),
                    "last_refill_time".to_string(),
                ],
            )
            .await
            .unwrap();

        // Vérifier que le `TokenBucket` est correctement créé à partir du cache
        let bucket = TokenBucket::from_cache(bucket_from_cache);
        assert_eq!(bucket.capacity, 100);
        assert_eq!(bucket.tokens, 99);

        cache
            .client
            .delete(&format!("ratelimit:{}", id))
            .await
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_big_consume_tokens() {
        let cache = TokenBucketsCacheRedis::new(CLIENT.clone());

        let id = "test-6";
        cache.create_bucket(id).await.unwrap();

        cache.consume_tokens(id, &Method::POST).await.unwrap();

        // Récupérer les données du bucket depuis Redis
        let bucket_from_cache = cache
            .client
            .hget_multiple(
                &format!("ratelimit:{}", id),
                vec![
                    "capacity".to_string(),
                    "tokens".to_string(),
                    "last_refill_time".to_string(),
                ],
            )
            .await
            .unwrap();

        // Vérifier que le `TokenBucket` est correctement créé à partir du cache
        let bucket = TokenBucket::from_cache(bucket_from_cache);
        assert_eq!(bucket.capacity, 100);
        assert_eq!(bucket.tokens, 95);

        cache
            .client
            .delete(&format!("ratelimit:{}", id))
            .await
            .unwrap();
    }
}
