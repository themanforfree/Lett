use crate::database::models::{article, establish_connection};

use super::{HyperReq, HyperRes};

pub async fn handle(_req: HyperReq, year: i32, month: u32) -> HyperRes {
    let articles =
        article::read_by_archive(&establish_connection(), year, month).unwrap_or_default();
    HyperRes::new(hyper::Body::from(format!("{:#?}", articles)))
}
