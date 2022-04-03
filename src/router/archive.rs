use super::{HyperReq, HyperRes};

pub async fn handle(_req: HyperReq, year: i32, month: i32) -> HyperRes {
    HyperRes::new(hyper::Body::from(format!(
        "This is the archive for {}/{}",
        year, month
    )))
}
