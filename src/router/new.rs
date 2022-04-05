use super::{HyperReq, HyperRes};
use crate::database::models::{
    article::{self, NewArticle},
    establish_connection,
};

pub async fn handle(req: HyperReq) -> HyperRes {
    let body = hyper::body::to_bytes(req.into_body()).await.unwrap();
    let article = NewArticle::from(body);
    let insert_num = article::create(&establish_connection(), &article).unwrap_or_default();
    HyperRes::new(hyper::Body::from(format!("{:#?}\n{}", article, insert_num)))
}