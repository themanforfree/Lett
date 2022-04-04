use super::{HyperReq, HyperRes};
use crate::database::models::{article::search_articles, establish_connection};

pub async fn handle(req: HyperReq) -> HyperRes {
    let query = req.uri().query().unwrap_or_default();
    let keyword = serde_urlencoded::from_str::<Vec<(String, String)>>(query)
        .unwrap_or_default()
        .iter()
        .find_map(|(k, v)| {
            if k == "keyword" {
                Some(v.to_owned())
            } else {
                None
            }
        })
        .unwrap_or_default();
    let articles = search_articles(&establish_connection(), &keyword).unwrap_or_default();
    HyperRes::new(format!("{:#?}", articles).into())
}
