use crate::database::models::{article, establish_connection};
use hyper::{Body, Request, Response};

pub(crate) async fn handle(_req: Request<Body>) -> Option<Response<Body>> {
    log::debug!("Request index page");
    let articles = article::read(&establish_connection()).unwrap_or_default();
    Some(Response::new(Body::from(format!("{:#?}", articles))))
}
