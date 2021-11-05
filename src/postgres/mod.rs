pub mod todos;

use super::config::database::DatabaseConfig;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use r2d2::PooledConnection;
pub struct Postgres {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Postgres {
    pub fn new(db_config: &DatabaseConfig) -> Postgres {
        let manager = ConnectionManager::<PgConnection>::new(&db_config.url);
        let pool = Pool::builder()
            .max_size(db_config.pool_size)
            .build(manager)
            .expect("Failed to create pool.");

        Postgres { pool }
    }

    pub fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool.get().unwrap()
    }
}
