use super::{HyperReq, HyperRes};
use crate::database::models::{article::read_articles, establish_connection};

pub async fn handle(_req: HyperReq) -> HyperRes {
    let articles = read_articles(&establish_connection()).unwrap();
    HyperRes::new(hyper::Body::from(format!("{:#?}", articles)))
}
