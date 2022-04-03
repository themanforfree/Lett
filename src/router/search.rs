use super::{HyperReq, HyperRes};

pub async fn handle(_req: HyperReq) -> HyperRes {
    HyperRes::new(hyper::Body::from("Search Page"))
}
