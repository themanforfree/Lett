use super::{HyperReq, HyperRes};
use crate::database::models::{article, establish_connection};

pub async fn handle(req: HyperReq) -> HyperRes {
    let body = hyper::body::to_bytes(req.into_body()).await.unwrap();
    let aid = serde_urlencoded::from_bytes::<Vec<(String, String)>>(body.as_ref())
        .unwrap()
        .iter()
        .find_map(|(k, v)| if k == "aid" { Some(v.parse().ok()?) } else { None })
        .unwrap_or_default();
    let n = article::delete(&establish_connection(), aid).unwrap_or_default();
    HyperRes::new(hyper::Body::from(format!("{}", n)))
}
