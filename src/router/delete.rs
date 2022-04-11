use crate::database::{
    establish_connection,
    models::{article, session},
};
use hyper::{header, Body, Request, Response, StatusCode};
use serde::Deserialize;

#[derive(Deserialize)]
struct Params {
    aid: u32,
}

pub(crate) async fn handle(req: Request<Body>) -> Option<Response<Body>> {
    log::debug!("Post to delete");
    let conn = establish_connection();
    let tmp = session::get_from_request(&conn, &req);
    match tmp {
        Some(s) if s.check_expiration() => {
            let body = hyper::body::to_bytes(req.into_body()).await.ok()?;
            let aid = serde_urlencoded::from_bytes::<Params>(&body).ok()?.aid;
            let n = article::delete(&conn, aid).unwrap_or_default();
            match n {
                0 => log::debug!("Delete article failed, aid = {}", aid),
                _ => log::debug!("Delete article success, aid = {}", aid),
            };
            Some(Response::new(Body::from(format!("{}", n))))
        }
        _ => {
            log::debug!("Post to delete failed, Redirect to /login");
            let mut res = Response::new(Body::empty());
            *res.status_mut() = StatusCode::FOUND;
            res.headers_mut().insert(
                header::LOCATION,
                header::HeaderValue::from_str("/login").unwrap(),
            );
            Some(res)
        }
    }
}
