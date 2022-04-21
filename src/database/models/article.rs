use crate::{config::CONFIG, database::schema::articles};
use anyhow::Result;
use diesel::prelude::*;
use hyper::body::Bytes;
use serde::{Deserialize, Serialize};
use time::{macros::format_description, OffsetDateTime};

#[derive(Queryable, QueryableByName, Debug, Serialize, AsChangeset, Deserialize)]
#[table_name = "articles"]

pub struct Article {
    pub aid: u32,
    pub title: String,
    pub content: String,
    pub created: i64,
    #[serde(default)]
    pub published: bool,
    pub comments_num: i32,
}

impl From<Bytes> for Article {
    fn from(body: Bytes) -> Self {
        serde_urlencoded::from_bytes(&body).unwrap()
    }
}

#[derive(Insertable, AsChangeset, Deserialize, Debug)]
#[table_name = "articles"]
pub struct NewArticle {
    pub title: String,
    pub content: String,
    #[serde(default = "default_created")]
    pub created: i64,
    #[serde(default)]
    pub published: bool,
}

fn default_created() -> i64 {
    OffsetDateTime::now_utc().unix_timestamp()
}

impl From<Bytes> for NewArticle {
    fn from(body: Bytes) -> Self {
        serde_urlencoded::from_bytes(&body).unwrap()
    }
}

pub fn read_by_id(conn: &MysqlConnection, id: u32) -> Result<Article> {
    use crate::database::schema::articles::dsl::*;
    articles.find(id).first::<Article>(conn).map_err(Into::into)
}

pub fn read_published(conn: &MysqlConnection) -> Result<Vec<Article>> {
    use crate::database::schema::articles::dsl::*;
    articles
        .filter(published.eq(true))
        .order(created.desc())
        .load::<Article>(conn)
        .map_err(Into::into)
}

pub fn read_all(conn: &MysqlConnection) -> Result<Vec<Article>> {
    use crate::database::schema::articles::dsl::*;
    articles
        .order(created.desc())
        .load::<Article>(conn)
        .map_err(Into::into)
}

fn get_start_and_end_of_month(year: i32, month: u8) -> Result<(i64, i64)> {
    let days = |year: i32, month: u8| -> u8 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                    29
                } else {
                    28
                }
            }
            _ => 255,
        }
    };

    let cfg = CONFIG.get().unwrap();
    let fmt = format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour]:[offset_minute]"
    );

    let start_timestamp = OffsetDateTime::parse(
        &format!(
            "{}-{:02}-01 00:00:00 {}",
            year, month, cfg.application.timezone
        ),
        fmt,
    )?
    .unix_timestamp();

    let end_timestamp = OffsetDateTime::parse(
        &format!(
            "{}-{:02}-{} 23:59:59 {}",
            year,
            month,
            days(year, month),
            cfg.application.timezone
        ),
        fmt,
    )?
    .unix_timestamp();
    Ok((start_timestamp, end_timestamp))
}

pub fn read_by_archive(conn: &MysqlConnection, year: i32, month: u8) -> Result<Vec<Article>> {
    use crate::database::schema::articles::dsl::*;
    let (start, end) = get_start_and_end_of_month(year, month)?;
    articles
        .filter(created.between(start, end))
        .order(aid.desc())
        .load::<Article>(conn)
        .map_err(Into::into)
}

pub fn search(conn: &MysqlConnection, keyword: &str) -> Result<Vec<Article>> {
    use crate::database::schema::articles::dsl::*;
    articles
        .filter(content.like(format!("%{}%", keyword)))
        .order(aid.desc())
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

pub fn update(conn: &MysqlConnection, article: &Article) -> Result<usize> {
    use crate::database::schema::articles::dsl::*;
    diesel::update(articles.find(article.aid))
        .set(article)
        .execute(conn)
        .map_err(Into::into)
}
