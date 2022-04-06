use crate::database::schema::articles;
use anyhow::Result;
use diesel::prelude::*;
use hyper::body::Bytes;
use serde::Deserialize;
use std::collections::HashMap;
use time::{Month, OffsetDateTime, Time, UtcOffset};

#[derive(Queryable, QueryableByName, Debug)]
#[table_name = "articles"]
#[allow(dead_code)]
pub(crate) struct Article {
    pub(crate) aid: u32,
    pub(crate) title: String,
    pub(crate) content: String,
    pub(crate) created: i64,
    pub(crate) published: bool,
    pub(crate) comments_num: i32,
}

#[derive(Insertable, AsChangeset, Deserialize, Debug)]
#[table_name = "articles"]
pub(crate) struct NewArticle {
    title: String,
    content: String,
    created: i64,
}

impl From<Bytes> for NewArticle {
    fn from(body: Bytes) -> Self {
        let query: HashMap<&str, &str> = serde_urlencoded::from_bytes(&body).unwrap();
        NewArticle {
            title: query["title"].to_string(),
            content: query["content"].to_string(),
            created: OffsetDateTime::now_utc().unix_timestamp(),
        }
    }
}

pub(crate) fn read(conn: &MysqlConnection) -> Result<Vec<Article>> {
    use crate::database::schema::articles::dsl::*;
    articles
        .order(aid.desc())
        .load::<Article>(conn)
        .map_err(Into::into)
}

fn get_start_and_end_of_month(year: i32, month: u8) -> Result<(i64, i64)> {
    let days = |year: i32, month: u8| -> u8 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if year % 4 == 0 {
                    29
                } else {
                    28
                }
            }
            _ => 255,
        }
    };
    let start_timestamp = OffsetDateTime::UNIX_EPOCH
        .replace_offset(UtcOffset::from_hms(8, 0, 0)?)
        .replace_year(year)?
        .replace_month(Month::try_from(month)?)?
        .replace_day(1)?
        .unix_timestamp();
    let end_timestamp = OffsetDateTime::UNIX_EPOCH
        .replace_offset(UtcOffset::from_hms(8, 0, 0)?)
        .replace_year(year)?
        .replace_month(Month::try_from(month)?)?
        .replace_day(days(year, month))?
        .replace_time(Time::from_hms(23, 59, 59)?)
        .unix_timestamp();
    Ok((start_timestamp, end_timestamp))
}

pub(crate) fn read_by_archive(
    conn: &MysqlConnection,
    year: i32,
    month: u8,
) -> Result<Vec<Article>> {
    use crate::database::schema::articles::dsl::*;
    let (start, end) = get_start_and_end_of_month(year, month)?;
    articles
        .filter(created.between(start, end))
        .load::<Article>(conn)
        .map_err(Into::into)
}

pub(crate) fn search(conn: &MysqlConnection, keyword: &str) -> Result<Vec<Article>> {
    use crate::database::schema::articles::dsl::*;
    articles
        .filter(content.like(format!("%{}%", keyword)))
        .load::<Article>(conn)
        .map_err(Into::into)
}

pub(crate) fn create(conn: &MysqlConnection, article: &NewArticle) -> Result<usize> {
    use crate::database::schema::articles::dsl::*;
    diesel::insert_into(articles)
        .values(article)
        .execute(conn)
        .map_err(Into::into)
}

pub(crate) fn delete(conn: &MysqlConnection, id: u32) -> Result<usize> {
    use crate::database::schema::articles::dsl::*;
    diesel::delete(articles.filter(aid.eq(id)))
        .execute(conn)
        .map_err(Into::into)
}
