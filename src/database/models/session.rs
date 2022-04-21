use crate::database::schema::sessions;
use anyhow::Result;
use diesel::prelude::*;
use hyper::{Body, Request};
use nanoid::nanoid;
use serde::Deserialize;
use time::{format_description::well_known::Rfc2822, OffsetDateTime};

#[derive(Queryable, Insertable, Deserialize, AsChangeset, Debug)]
#[table_name = "sessions"]
pub struct Session {
    sid: String,
    data: Option<String>,
    expiration: i64,
}

impl Session {
    pub fn new() -> Self {
        Self {
            sid: nanoid!(16),
            data: None,
            expiration: OffsetDateTime::now_utc().unix_timestamp() + 1800,
        }
    }

    pub fn to_cookie(&self) -> Result<String> {
        Ok(format!(
            "SESSIONID={}; Expires={};  path=/",
            self.sid,
            OffsetDateTime::from_unix_timestamp(self.expiration)?.format(&Rfc2822)?
        ))
    }

    pub fn check_expiration(&self) -> bool {
        OffsetDateTime::now_utc().unix_timestamp() < self.expiration
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

pub fn read_all(conn: &MysqlConnection) -> Result<Vec<Session>> {
    use crate::database::schema::sessions::dsl::*;
    sessions.load::<Session>(conn).map_err(Into::into)
}

fn get(conn: &MysqlConnection, id: &str) -> Result<Session> {
    use crate::database::schema::sessions::dsl::*;
    sessions.find(id).first::<Session>(conn).map_err(Into::into)
}

pub fn insert(conn: &MysqlConnection, session: &Session) -> Result<usize> {
    use crate::database::schema::sessions::dsl::*;
    diesel::insert_into(sessions)
        .values(session)
        .execute(conn)
        .map_err(Into::into)
}

pub fn get_from_request(conn: &MysqlConnection, req: &Request<Body>) -> Option<Session> {
    let cookie = req.headers().get("Cookie")?.to_str().ok()?;
    let id = cookie.split(';').find_map(|s| {
        if s.trim().starts_with("SESSIONID=") {
            s.split_once('=').map(|(_, s)| s.trim())
        } else {
            None
        }
    })?;
    get(conn, id).ok()
}
