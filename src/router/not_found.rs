use super::{HyperReq, HyperRes};

pub async fn handler(_req: HyperReq) -> HyperRes {
    HyperRes::new(hyper::Body::from("Not found"))
}
