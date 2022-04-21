use crate::{
    database::{
        establish_connection,
        models::{
            article::{self, Article},
            session,
        },
    },
    router::ADMIN_TEMPLATES,
};
use hyper::{header, Body, Method, Request, Response, StatusCode};
use matchit::Params;
use serde::Deserialize;
use tera::Context;

#[derive(Deserialize)]
struct UpdateParams {
    aid: u32,
}

// pub async fn handle(req: Request<Body>) -> Option<Response<Body>> {

pub async fn handle(req: Request<Body>, _params: Params<'_, '_>) -> Option<Response<Body>> {
    let conn = establish_connection();
    let tmp = session::get_from_request(&conn, &req);
    match tmp {
        Some(s) if s.check_expiration() => match *req.method() {
            Method::POST => {
                log::debug!("Post to Update");
                let body = hyper::body::to_bytes(req.into_body()).await.ok()?;
                let article = Article::from(body);
                match article::update(&conn, &article) {
                    Ok(_) => Some(Response::new(Body::from("Update article success"))),
                    Err(_) => Some(Response::new(Body::from("Update article failed"))),
                }
            }
            Method::GET => {
                log::debug!("Request Update page");
                let query = req.uri().query()?;
                let params: UpdateParams = serde_urlencoded::from_str(query).ok()?;
                let atc = article::read_by_id(&conn, params.aid).ok()?;
                let mut ctx = Context::new();
                ctx.insert("article", &atc);
                let body = ADMIN_TEMPLATES.get()?.render("update.html", &ctx).unwrap();
                Some(Response::new(hyper::Body::from(body)))
            }
            _ => None,
        },
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
