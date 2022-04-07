use crate::database::models::{article, establish_connection};
use hyper::{Body, Request, Response};

pub(crate) async fn handle(_req: Request<Body>, year: &str, month: &str) -> Option<Response<Body>> {
    if year.len() != 4 || month.len() != 2 {
        return None;
    }
    let year = year.parse().ok()?;
    let month = month.parse().ok()?;
    if !(1..=12).contains(&month) {
        return None;
    }
    log::debug!("Request archive page: year = {}, month = {}", year, month);
    let articles = article::read_by_archive(&establish_connection(), year, month).ok()?;
    Some(Response::new(Body::from(format!("{:#?}", articles))))
}
