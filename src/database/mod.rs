use anyhow::{anyhow, Result};
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    MysqlConnection,
};
use dotenv::dotenv;
use once_cell::sync::OnceCell;
use std::env;

pub(crate) mod models;
mod schema;

type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
static CONNECTION_POOL: OnceCell<MysqlPool> = OnceCell::new();

embed_migrations!();

pub(crate) fn init() -> Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
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
