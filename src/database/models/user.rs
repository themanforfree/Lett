use super::CRUD;
use crate::database::schema::users;
use chrono::NaiveDateTime;
use diesel::{prelude::*, result::Error};
#[derive(Queryable)]
pub struct User {
    pub uid: u32,
    pub username: String,
    pub password: String,
    pub created: NaiveDateTime,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

impl CRUD<NewUser, u32> for User {
    fn create(conn: &MysqlConnection, from: &NewUser) -> Result<Self, Error> {
        diesel::insert_into(users::table)
            .values(from)
            .execute(conn)?;
        Ok(users::table.order(users::uid.desc()).first(conn)?)
    }
    fn read(conn: &MysqlConnection) -> Vec<Self> {
        users::table.load::<Self>(conn).unwrap_or(Vec::new())
    }

    fn update(conn: &MysqlConnection, id: u32, value: &NewUser) -> Result<Self, Error> {
        diesel::update(users::table.find(id))
            .set(value)
            .execute(conn)?;
        Ok(users::table.find(id).first::<Self>(conn)?)
    }

    fn delete(conn: &MysqlConnection, id: u32) -> Result<usize, Error> {
        diesel::delete(users::table.find(id)).execute(conn)
    }

    fn get_by_pk(conn: &MysqlConnection, id: u32) -> Result<Self, diesel::result::Error> {
        users::table.find(id).first::<Self>(conn)
    }
}
