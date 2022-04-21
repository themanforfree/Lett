use anyhow::{anyhow, Result};
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    MysqlConnection,
};

use once_cell::sync::OnceCell;

use crate::config::CONFIG;

use self::models::{article, comment, session};

pub mod models;
pub mod schema;

type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
static CONNECTION_POOL: OnceCell<MysqlPool> = OnceCell::new();

pub fn init() -> Result<()> {
    let cfg = CONFIG.get().unwrap();
    let manager = ConnectionManager::<MysqlConnection>::new(&cfg.database.url);
    let pool: MysqlPool = Pool::builder().test_on_check_out(true).build(manager)?;
    let conn = pool.get()?;
    article::read_all(&conn)?;
    session::read_all(&conn)?;
    comment::read_all(&conn)?;
    CONNECTION_POOL
        .set(pool)
        .map_err(|_| anyhow!("CONNECTION_POOL set failed"))
}

pub fn establish_connection() -> PooledConnection<ConnectionManager<MysqlConnection>> {
    CONNECTION_POOL
        .get()
        .expect("failed to get mysql connection pool")
        .get()
        .expect("failed to get mysql connection")
}
