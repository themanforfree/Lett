use crate::database::models::{article, establish_connection};
use hyper::{Body, Request, Response};

pub(crate) async fn handle(_req: Request<Body>, year: &str, month: &str) -> Option<Response<Body>> {
    let year = year.parse().ok()?;
    let month = month.parse().ok()?;
    if month > 12 || month < 1 {
        return None;
    }
    let articles = article::read_by_archive(&establish_connection(), year, month).ok()?;
    Some(Response::new(Body::from(format!("{:#?}", articles))))
}
