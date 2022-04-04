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

pub fn read_articles(conn: &MysqlConnection) -> Result<Vec<Article>> {
    use crate::database::schema::articles::dsl::*;
    articles
        .order(aid.desc())
        .load::<Article>(conn)
        .map_err(Into::into)
}

pub fn read_articles_by_archive(
    conn: &MysqlConnection,
    year: i32,
    month: u32,
) -> Result<Vec<Article>> {
    diesel::sql_query(
        "SELECT * FROM articles WHERE year(created) = ? AND month(created) = ?  ORDER BY aid DESC",
    )
    .bind::<diesel::sql_types::Text, _>(year.to_string())
    .bind::<diesel::sql_types::Text, _>(month.to_string())
    .load::<Article>(conn)
    .map_err(Into::into)
}

pub fn search_articles(conn: &MysqlConnection, keyword: &str) -> Result<Vec<Article>> {
    use crate::database::schema::articles::dsl::*;
    articles
        .filter(content.like(format!("%{}%", keyword)))
        .load::<Article>(conn)
        .map_err(Into::into)
}

pub fn add_article(conn: &MysqlConnection, article: &NewArticle) -> Result<usize> {
    use crate::database::schema::articles::dsl::*;
    diesel::insert_into(articles)
        .values(article)
        .execute(conn)
        .map_err(Into::into)
}
