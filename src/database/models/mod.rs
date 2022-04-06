use diesel::{prelude::*, MysqlConnection};
use dotenv::dotenv;
use std::env;

pub(crate) mod article;
pub(crate) mod session;

pub(crate) fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
