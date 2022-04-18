use crate::database::schema::comments;
use anyhow::Result;
use diesel::prelude::*;
use hyper::body::Bytes;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Queryable, QueryableByName, Debug, Serialize, AsChangeset, Deserialize)]
#[table_name = "comments"]
pub struct Comment {
    cid: u32,
    aid: u32,
    author: String,
    email: String,
    url: Option<String>,
    text: String,
    created: i64,
}

#[derive(Insertable, AsChangeset, Deserialize, Debug)]
#[table_name = "comments"]
pub struct NewComment {
    aid: u32,
    author: String,
    email: String,
    #[serde(default = "Option::default")]
    url: Option<String>,
    text: String,
    #[serde(default = "default_created")]
    created: i64,
}

impl From<Bytes> for NewComment {
    fn from(body: Bytes) -> Self {
        serde_urlencoded::from_bytes(&body).unwrap()
    }
}

fn default_created() -> i64 {
    OffsetDateTime::now_utc().unix_timestamp()
}

pub fn read_by_aid(conn: &MysqlConnection, id: u32) -> Result<Vec<Comment>> {
    use crate::database::schema::comments::dsl::*;
    comments
        .filter(aid.eq(id))
        .load::<Comment>(conn)
        .map_err(Into::into)
}

pub fn create(conn: &MysqlConnection, comment: &NewComment) -> Result<usize> {
    use crate::database::schema::comments::dsl::*;
    diesel::insert_into(comments)
        .values(comment)
        .execute(conn)
        .map_err(Into::into)
}

// pub fn delete(conn: &MysqlConnection, id: u32) -> Result<usize> {
//     use crate::database::schema::comments::dsl::*;
//     diesel::delete(comments.find(id))
//         .execute(conn)
//         .map_err(Into::into)
// }
