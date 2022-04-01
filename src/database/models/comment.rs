use super::Crud;
use crate::database::schema::comments;
use chrono::NaiveDateTime;
use diesel::{prelude::*, result::Error};

#[derive(Queryable)]
pub struct Comment {
    pub cid: u32,
    pub aid: Option<u32>,
    pub created: NaiveDateTime,
    pub author_id: u32,
    pub owner_id: Option<u32>,
    pub text: String,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "comments"]
pub struct NewComment {
    pub aid: Option<u32>,
    pub author_id: u32,
    pub owner_id: Option<u32>,
    pub text: String,
}

impl Crud<NewComment, u32> for Comment {
    fn create(conn: &MysqlConnection, from: &NewComment) -> Result<Self, Error> {
        diesel::insert_into(comments::table)
            .values(from)
            .execute(conn)?;
        comments::table
            .order(comments::cid.desc())
            .first::<Self>(conn)
    }
    fn read(conn: &MysqlConnection) -> Vec<Self> {
        comments::table.load::<Self>(conn).unwrap_or_default()
    }

    fn update(conn: &MysqlConnection, id: u32, value: &NewComment) -> Result<Self, Error> {
        diesel::update(comments::table.find(id))
            .set(value)
            .execute(conn)?;
        comments::table.find(id).first::<Self>(conn)
    }

    fn delete(conn: &MysqlConnection, id: u32) -> Result<usize, Error> {
        diesel::delete(comments::table.find(id)).execute(conn)
    }

    fn get_by_pk(conn: &MysqlConnection, id: u32) -> Result<Self, diesel::result::Error> {
        comments::table.find(id).first::<Self>(conn)
    }
}
