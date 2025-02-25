use crate::redis::{RedisClient, RedisRepository, RedisRepositoryResult};
use chrono::{DateTime, Utc};

/// The refill rate in tokens per second.
const REFILL_RATE_PER_SECOND: i64 = 10;
/// The default maximum capacity of the token bucket.
const DEFAULT_CAPACITY: u64 = 100;

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
    async fn save_bucket(&self, id: &str, bucket: &TokenBucket) -> RedisRepositoryResult<()>;
    /// Creates a new token bucket with default values in Redis.
    async fn create_bucket(&self, id: &str) -> RedisRepositoryResult<()>;
    /// Refills an existing token bucket and updates Redis.
    async fn refill_bucket(&self, id: &str) -> RedisRepositoryResult<()>;
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
    async fn save_bucket(&self, id: &str, bucket: &TokenBucket) -> RedisRepositoryResult<()> {
        self.client
            .hset_multiple(&format!("{}:{}", self.prefix, id), bucket.into())
            .await
    }

    /// Creates a new token bucket in Redis with default values.
    ///
    /// # Arguments
    /// * `id` - The unique identifier for the token bucket (ip or uuid).
    async fn create_bucket(&self, id: &str) -> RedisRepositoryResult<()> {
        self.client
            .hset_multiple(
                &format!("{}:{}", self.prefix, id),
                TokenBucket::default().into(),
            )
            .await
    }

    /// Refills an existing token bucket and updates Redis.
    ///
    /// # Arguments
    /// * `id` - The unique identifier for the token bucket (ip or uuid).
    async fn refill_bucket(&self, id: &str) -> RedisRepositoryResult<()> {
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

        let mut bucket = TokenBucket::from_cache(bucket_from_cache);
        bucket.refill();
        self.save_bucket(id, &bucket).await
    }
}
