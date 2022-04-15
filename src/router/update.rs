use crate::database::{
    establish_connection,
    models::{
        article::{self, Article},
        session,
    },
};
use hyper::{header, Body, Request, Response, StatusCode};

pub(crate) async fn handle(req: Request<Body>) -> Option<Response<Body>> {
    log::debug!("Post to Update");
    let conn = establish_connection();
    let tmp = session::get_from_request(&conn, &req);
    match tmp {
        Some(s) if s.check_expiration() => {
            let body = hyper::body::to_bytes(req.into_body()).await.ok()?;
            let article = Article::from(body);

            match article::update(&conn, &article) {
                Ok(_) => Some(Response::new(Body::from("Update article success"))),
                Err(_) => Some(Response::new(Body::from("Update article failed"))),
            }
        }
        _ => {
            log::debug!("Post to update failed, Redirect to /login");
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
