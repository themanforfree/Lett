use crate::database::schema::articles;
use anyhow::Result;
use chrono::{Duration, NaiveDate, NaiveDateTime};
use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct Article {
    pub aid: u32,
    pub title: String,
    pub content: String,
    pub created: NaiveDateTime,
    pub published: bool,
    pub comments_num: i32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "articles"]
pub struct NewArticle {
    pub title: String,
    pub content: String,
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
    use crate::database::schema::articles::dsl::*;

    articles
        .filter(created.between(
            NaiveDate::from_ymd(year, month, 1).and_hms(0, 0, 0) - Duration::hours(8),
            NaiveDate::from_ymd(year, month + 1, 1).and_hms(0, 0, 0)
                - Duration::hours(8)
                - Duration::minutes(1),
        ))
        .order(aid.desc())
        .load::<Article>(conn)
        .map_err(Into::into)
}
