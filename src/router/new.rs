use crate::database::models::{
    article::{self, NewArticle},
    establish_connection, session,
};
use hyper::{header, Body, Request, Response, StatusCode};

pub(crate) async fn handle(req: Request<Body>) -> Option<Response<Body>> {
    let conn = establish_connection();
    let tmp = session::get_from_request(&conn, &req);
    match tmp {
        Some(s) if s.check_expiration() => {
            let body = hyper::body::to_bytes(req.into_body()).await.ok()?;

            let article = NewArticle::from(body);
            let insert_num = article::create(&conn, &article).unwrap_or_default();
            Some(Response::new(Body::from(format!(
                "{:#?}\n{}",
                article, insert_num
            ))))
        }
        _ => {
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
