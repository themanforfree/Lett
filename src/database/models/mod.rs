use diesel::{prelude::*, result::Error, MysqlConnection};
use dotenv::dotenv;
use std::env;

mod article;
mod comment;
mod settings;
mod user;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub trait Crud<NewModel, PK> {
    fn create(conn: &MysqlConnection, from: &NewModel) -> Result<Self, Error>
    where
        Self: Sized;

    fn read(conn: &MysqlConnection) -> Vec<Self>
    where
        Self: Sized;

    fn update(conn: &MysqlConnection, pk: PK, value: &NewModel) -> Result<Self, Error>
    where
        Self: Sized;

    fn delete(conn: &MysqlConnection, pk: PK) -> Result<usize, Error>
    where
        Self: Sized;

    fn get_by_pk(conn: &MysqlConnection, pk: PK) -> Result<Self, Error>
    where
        Self: Sized;
}
