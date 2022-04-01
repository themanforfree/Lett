use crate::database::schema::articles;
use chrono::NaiveDateTime;
use diesel::{prelude::*, result::Error};

use super::Crud;

#[derive(Queryable)]
pub struct Article {
    pub aid: u32,
    pub title: Option<String>,
    pub content: Option<String>,
    pub created: NaiveDateTime,
    pub modified: NaiveDateTime,
    pub author_id: Option<u32>,
    pub published: bool,
    pub comments_num: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "articles"]
struct NewArticle {
    pub title: Option<String>,
    pub content: Option<String>,
    pub modified: NaiveDateTime,
    pub author_id: Option<u32>,
}

impl Crud<NewArticle, u32> for Article {
    fn create(conn: &MysqlConnection, from: &NewArticle) -> Result<Self, Error> {
        diesel::insert_into(articles::table)
            .values(from)
            .execute(conn)?;
        articles::table
            .order(articles::aid.desc())
            .first::<Self>(conn)
    }
    fn read(conn: &MysqlConnection) -> Vec<Self> {
        articles::table.load::<Self>(conn).unwrap_or_default()
    }
    fn update(conn: &MysqlConnection, id: u32, value: &NewArticle) -> Result<Self, Error> {
        diesel::update(articles::table.find(&id))
            .set(value)
            .execute(conn)?;
        articles::table.find(id).first::<Self>(conn)
    }
    fn delete(conn: &MysqlConnection, id: u32) -> Result<usize, Error> {
        diesel::delete(articles::table.find(id)).execute(conn)
    }
    fn get_by_pk(conn: &MysqlConnection, id: u32) -> Result<Self, Error> {
        articles::table.find(id).first::<Self>(conn)
    }
}
