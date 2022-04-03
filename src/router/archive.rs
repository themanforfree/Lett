use crate::database::models::{article::read_articles_by_archive, establish_connection};

use super::{HyperReq, HyperRes};

pub async fn handle(_req: HyperReq, year: i32, month: u32) -> HyperRes {
    let articles = read_articles_by_archive(&establish_connection(), year, month).unwrap();
    HyperRes::new(hyper::Body::from(format!("{:#?}", articles)))
}
