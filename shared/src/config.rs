use std::env;

use crate::parse::{boolean, choices};

#[derive(Clone)]
pub struct RedisInfo {
    pub host: String,
    pub port: String,
    pub username: String,
    pub password: String,
}

impl RedisInfo {
    pub fn get_url(&self) -> String {
        if self.password.is_empty() {
            return format!("redis://{}:{}", self.host, self.port);
        }
        format!(
            "redis://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

#[derive(Clone)]
pub struct Config {
    pub development: bool,
    pub version: String,

    pub auth_driver: String,

    pub database_url: String,

    pub redis_info: RedisInfo,

    pub jwt_secret: String,
    pub jwt_expired_in: i64,

    pub refresh_token_ttl: i64,
    pub session_ttl: i64,
}

impl Config {
    pub fn init() -> Config {
        dotenv::dotenv().ok();

        let development = boolean()
            .parse::<bool>(
                env::var("DEVELOPMENT")
                    .expect("DEVELOPMENT must be set")
                    .parse()
                    .unwrap(),
            )
            .expect("DEVELOPMENT must be a boolean");
        let version = env::var("VERSION").expect("VERSION must be set");

        let auth_driver = choices(vec!["session", "jwt", "oauth"])
            .parse(env::var("AUTH_DRIVER").expect("AUTH_DRIVER must be set"))
            .expect("AUTH_DRIVER must be in choices");

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let redis_info = RedisInfo {
            host: env::var("REDIS_HOST").expect("REDIS_HOST must be set"),
            port: env::var("REDIS_PORT").expect("REDIS_PORT must be set"),
            username: env::var("REDIS_USERNAME").expect("REDIS_USERNAME must be set"),
            password: env::var("REDIS_PASSWORD").expect("REDIS_PASSWORD must be set"),
        };

        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expired_in = env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set");

        let refresh_token_ttl =
            env::var("REFRESH_TOKEN_TTL").expect("REFRESH_TOKEN_TTL must be set");

        let session_ttl = env::var("SESSION_TTL").expect("SESSION_TTL must be set");

        Config {
            development,
            version,
            auth_driver,
            database_url,
            redis_info,
            jwt_secret,
            jwt_expired_in: jwt_expired_in.parse::<i64>().unwrap(),
            refresh_token_ttl: refresh_token_ttl.parse::<i64>().unwrap(),
            session_ttl: session_ttl.parse::<i64>().unwrap(),
        }
    }
}
