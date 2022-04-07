use crate::database::models::{article, establish_connection};
use hyper::{Body, Request, Response};
use serde::Deserialize;

#[derive(Deserialize)]
struct Params {
    keyword: String,
}

pub(crate) async fn handle(req: Request<Body>) -> Option<Response<Body>> {
    let query = req.uri().query()?;
    let params: Params = serde_urlencoded::from_str(query).ok()?;
    let articles = article::search(&establish_connection(), &params.keyword).unwrap_or_default();
    Some(Response::new(format!("{:#?}", articles).into()))
}
