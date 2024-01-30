use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expired_in: i64,
}

impl Config {
    pub fn init() -> Config {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expired_in = env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set");
        Config {
            database_url,
            jwt_secret,
            jwt_expired_in: jwt_expired_in.parse::<i64>().unwrap(),
        }
    }
}