use crate::{
    database::{
        establish_connection,
        models::{
            article::{self, NewArticle},
            session,
        },
    },
    router::TEMPLATES,
};
use hyper::{header, Body, Method, Request, Response, StatusCode};
use matchit::Params;
use tera::Context;

pub async fn handle(req: Request<Body>, _params: Params<'_, '_>) -> Option<Response<Body>> {
    log::debug!("Post to new");
    let conn = establish_connection();
    let tmp = session::get_from_request(&conn, &req);
    match tmp {
        Some(s) if s.check_expiration() => match *req.method() {
            Method::POST => {
                let body = hyper::body::to_bytes(req.into_body()).await.ok()?;
                let article = NewArticle::from(body);
                if article.title.is_empty() || article.content.is_empty() {
                    return Some(Response::new(Body::from(format!("Crate article failed"))));
                };
                match article::create(&conn, &article) {
                    Ok(u) => Some(Response::new(Body::from(format!(
                        "Crate {} article success",
                        u
                    )))),
                    Err(_) => Some(Response::new(Body::from(format!("Crate article failed")))),
                }
            }
            Method::GET => {
                let ctx = Context::new();
                let body = TEMPLATES
                    .get()
                    .unwrap()
                    .render("admin/new.html", &ctx)
                    .unwrap();
                Some(Response::new(hyper::Body::from(body)))
            }
            _ => None,
        },
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
