use crate::database::schema::articles;
use anyhow::Result;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use hyper::body::Bytes;
use serde::Deserialize;

#[derive(Queryable, QueryableByName, Debug)]
#[table_name = "articles"]
pub struct Article {
    pub aid: u32,
    pub title: String,
    pub content: String,
    pub created: NaiveDateTime,
    pub published: bool,
    pub comments_num: i32,
}

#[derive(Insertable, AsChangeset, Deserialize, Debug)]
#[table_name = "articles"]
pub struct NewArticle {
    pub title: String,
    pub content: String,
}

impl From<Bytes> for NewArticle {
    fn from(body: Bytes) -> Self {
        let article: NewArticle = serde_urlencoded::from_bytes(body.as_ref()).unwrap();
        article
    }
}

pub fn read(conn: &MysqlConnection) -> Result<Vec<Article>> {
    use crate::database::schema::articles::dsl::*;
    articles
        .order(aid.desc())
        .load::<Article>(conn)
        .map_err(Into::into)
}

pub fn read_by_archive(conn: &MysqlConnection, year: i32, month: u32) -> Result<Vec<Article>> {
    let sql = format!(
        "SELECT * FROM articles WHERE year(CONVERT_TZ(`created`, '+00:00', '{timezone}')) = {} AND month(CONVERT_TZ(`created`, '+00:00', '{timezone}')) = {}  ORDER BY aid DESC",
        year, month, timezone = "+08:00"
    );
    diesel::sql_query(sql)
        .load::<Article>(conn)
        .map_err(Into::into)
}

pub fn search(conn: &MysqlConnection, keyword: &str) -> Result<Vec<Article>> {
    use crate::database::schema::articles::dsl::*;
    articles
        .filter(content.like(format!("%{}%", keyword)))
        .load::<Article>(conn)
        .map_err(Into::into)
}

pub fn create(conn: &MysqlConnection, article: &NewArticle) -> Result<usize> {
    use crate::database::schema::articles::dsl::*;
    diesel::insert_into(articles)
        .values(article)
        .execute(conn)
        .map_err(Into::into)
}

pub fn delete(conn: &MysqlConnection, id: u32) -> Result<usize> {
    use crate::database::schema::articles::dsl::*;
    diesel::delete(articles.filter(aid.eq(id)))
        .execute(conn)
        .map_err(Into::into)
}
