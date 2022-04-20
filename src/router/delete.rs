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

pub async fn handle(req: Request<Body>) -> Option<Response<Body>> {
    log::debug!("Post to delete");
    let conn = establish_connection();
    let tmp = session::get_from_request(&conn, &req);
    match tmp {
        Some(s) if s.check_expiration() => {
            let body = hyper::body::to_bytes(req.into_body()).await.ok()?;
            let aid = serde_urlencoded::from_bytes::<Params>(&body).ok()?.aid;
            match article::delete(&conn, aid) {
                Ok(n) if n == 0 => {
                    log::debug!("Delete article failed, aid = {}", aid);
                    Some(
                        Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from(format!("Not found article has id {}", aid)))
                            .unwrap(),
                    )
                }
                Ok(_) => {
                    log::debug!("Delete article success, aid = {}", aid);
                    Some(
                        Response::builder()
                            .status(StatusCode::OK)
                            .header(header::CONTENT_TYPE, "text/plain")
                            .body(Body::from("Delete article success"))
                            .unwrap(),
                    )
                }
                Err(e) => {
                    log::debug!("Delete article failed, aid = {}", aid);
                    Some(
                        Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .header(header::CONTENT_TYPE, "text/plain")
                            .body(Body::from(format!("{}", e)))
                            .unwrap(),
                    )
                }
            }
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
