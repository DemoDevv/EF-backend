use diesel::pg::PgConnection;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;

use api_configs::config::Config;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection(config: &Config) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(config.database_url.clone());
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");
    pool
}
