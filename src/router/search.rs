use crate::database::models::{article, establish_connection};
use hyper::{Body, Request, Response};

pub(crate) async fn handle(req: Request<Body>) -> Option<Response<Body>> {
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
    let articles = article::search(&establish_connection(), &keyword).unwrap_or_default();
    Some(Response::new(format!("{:#?}", articles).into()))
}
