use super::{HyperReq, HyperRes};
use crate::database::models::{article, establish_connection};

pub async fn handle(_req: HyperReq) -> HyperRes {
    let articles = article::read(&establish_connection()).unwrap_or_default();
    HyperRes::new(hyper::Body::from(format!("{:#?}", articles)))
}
