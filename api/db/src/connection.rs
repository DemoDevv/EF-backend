use std::sync::Arc;

use diesel::pg::PgConnection;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::CustomizeConnection;
use diesel::Connection;

#[derive(Debug, Clone, Copy)]
pub struct TestCustomizer;

impl<C: Connection> CustomizeConnection<C, diesel::r2d2::Error> for TestCustomizer {
    fn on_acquire(&self, conn: &mut C) -> Result<(), diesel::r2d2::Error> {
        conn.begin_test_transaction()
            .map_err(diesel::r2d2::Error::QueryError)
    }
}

use api_configs::config::Config;

pub type Pool = Arc<r2d2::Pool<ConnectionManager<PgConnection>>>;

/// This function establishes a connection to the database.
/// It creates a new connection pool with a maximum size of 10.
/// The pool is wrapped in an Arc to allow it to be shared across threads.
/// The connection pool is created using the database URL from the configuration.
/// If the pool fails to be created, the function will panic.
pub fn establish_connection(config: &Config) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(config.database_url.clone());
    let pool: Pool = Arc::new(
        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool"),
    );
    pool
}

/// This function establishes a connection to the database for testing purposes.
/// It creates a new connection pool with a maximum size of 1 and a customizer that
/// begins a test transaction on each connection acquisition.
pub fn establish_testing_connection(config: &Config) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(config.database_test_url.clone());
    let pool: Pool = Arc::new(
        r2d2::Pool::builder()
            .test_on_check_out(true)
            .max_size(1)
            .connection_customizer(Box::new(TestCustomizer))
            .build(manager)
            .expect("Failed to create testing pool"),
    );
    pool
}
