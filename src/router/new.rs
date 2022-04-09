use crate::database::models::{
    article::{self, NewArticle},
    establish_connection, session,
};
use hyper::{header, Body, Request, Response, StatusCode};

pub(crate) async fn handle(req: Request<Body>) -> Option<Response<Body>> {
    log::debug!("Post to new");
    let conn = establish_connection();
    let tmp = session::get_from_request(&conn, &req);
    match tmp {
        Some(s) if s.check_expiration() => {
            let body = hyper::body::to_bytes(req.into_body()).await.ok()?;
            let article = NewArticle::from(body);

            match article::create(&conn, &article) {
                Ok(u) => Some(Response::new(Body::from(format!(
                    "Crate {} article success",
                    u
                )))),
                Err(_) => Some(Response::new(Body::from(format!("Crate article failed")))),
            }
            // log::debug!("New article: {:?}", article);
            // Some(Response::new(Body::from(format!("{:#?}", article,))))
        }
        _ => {
            log::debug!("Post to new failed, Redirect to /login");
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
