use crate::redis::{RedisClient, RedisRepository, RedisRepositoryResult};

/// The refill rate in tokens per second.
const REFILL_RATE_PER_SECOND: i64 = 10;
/// The default maximum capacity of the token bucket.
const DEFAULT_CAPACITY: u64 = 100;

#[derive(Clone)]
pub struct TokenBucket {
    capacity: u64,
    tokens: u64,
    last_refill_time: chrono::DateTime<chrono::Utc>,
}

impl Default for TokenBucket {
    fn default() -> Self {
        // TODO: ajouter dans le .env
        TokenBucket {
            capacity: DEFAULT_CAPACITY,
            tokens: 100,
            last_refill_time: chrono::Utc::now(),
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

impl From<TokenBucket> for Vec<(String, String)> {
    fn from(bucket: TokenBucket) -> Self {
        _from(&bucket)
    }
}

impl From<&TokenBucket> for Vec<(String, String)> {
    fn from(bucket: &TokenBucket) -> Self {
        _from(bucket)
    }
}

impl TokenBucket {
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
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.to_utc())
                .unwrap_or_else(chrono::Utc::now),
        }
    }

    fn refill(&mut self) {
        let now = chrono::Utc::now();
        let elapsed = now.signed_duration_since(self.last_refill_time);

        let tokens_to_add = (elapsed.num_seconds() * REFILL_RATE_PER_SECOND).max(0);
        self.tokens = self
            .tokens
            .saturating_add(tokens_to_add as u64)
            .min(self.capacity);
        self.last_refill_time = now;
    }
}

#[async_trait::async_trait]
pub trait TokenBucketsCache: Clone + Send + Sync + 'static {
    async fn save_bucket(&self, id: &str, bucket: &TokenBucket) -> RedisRepositoryResult<()>;
    async fn create_bucket(&self, id: &str) -> RedisRepositoryResult<()>;
    async fn refill_bucket(&self, id: &str) -> RedisRepositoryResult<()>;
}

#[derive(Clone)]
pub struct TokenBucketsCacheRedis {
    client: RedisClient,
    prefix: String,
}

impl TokenBucketsCacheRedis {
    pub fn new(client: RedisClient) -> Self {
        TokenBucketsCacheRedis {
            client,
            prefix: "ratelimit".to_string(),
        }
    }
}

#[async_trait::async_trait]
impl TokenBucketsCache for TokenBucketsCacheRedis {
    async fn save_bucket(&self, id: &str, bucket: &TokenBucket) -> RedisRepositoryResult<()> {
        self.client
            .hset_multiple(&format!("{}:{}", self.prefix, id), bucket.into())
            .await
    }

    async fn create_bucket(&self, id: &str) -> RedisRepositoryResult<()> {
        self.client
            .hset_multiple(
                &format!("{}:{}", self.prefix, id),
                TokenBucket::default().into(),
            )
            .await
    }

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
