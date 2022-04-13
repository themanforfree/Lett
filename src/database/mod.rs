use anyhow::{anyhow, Result};
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    MysqlConnection,
};

use once_cell::sync::OnceCell;

use crate::config;

pub(crate) mod models;
mod schema;

type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
static CONNECTION_POOL: OnceCell<MysqlPool> = OnceCell::new();

embed_migrations!();

pub(crate) fn init(cfg: config::Database) -> Result<()> {
    let manager = ConnectionManager::<MysqlConnection>::new(cfg.url);
    let pool: MysqlPool = Pool::builder().test_on_check_out(true).build(manager)?;
    embedded_migrations::run(&pool.get()?)?;
    CONNECTION_POOL
        .set(pool)
        .map_err(|_| anyhow!("CONNECTION_POOL set failed"))
}

pub(crate) fn establish_connection() -> PooledConnection<ConnectionManager<MysqlConnection>> {
    CONNECTION_POOL
        .get()
        .expect("failed to get mysql connection pool")
        .get()
        .expect("failed to get mysql connection")
}
