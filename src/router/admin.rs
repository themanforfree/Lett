use crate::{
    database::{
        establish_connection,
        models::{article, comment, session},
    },
    router::ADMIN_TEMPLATES,
};
use hyper::{header, Body, Request, Response, StatusCode};
use matchit::Params;
use tera::Context;

pub async fn handle(req: Request<Body>, params: Params<'_, '_>) -> Option<Response<Body>> {
    let path = params.get("path").unwrap_or("");
    match session::get_from_request(&establish_connection(), &req) {
        Some(s) if s.check_expiration() => {
            let tera = ADMIN_TEMPLATES.get().unwrap();
            log::debug!("Request admin page success: {:?}", s);
            let body = match path {
                "" | "index" => tera.render("index.html", &Context::new()).unwrap(),
                "posts" => {
                    let mut ctx = Context::new();
                    let articles = article::read_all(&establish_connection()).unwrap();
                    ctx.insert("is_posts", &true);
                    ctx.insert("contents", &articles);
                    tera.render("list.html", &ctx).unwrap()
                }
                "comments" => {
                    let mut ctx = Context::new();
                    let comments = comment::read_all(&establish_connection()).unwrap();
                    ctx.insert("is_comments", &true);
                    ctx.insert("contents", &comments);
                    tera.render("list.html", &ctx).unwrap()
                }

                _ => return None,
            };
            Some(Response::new(Body::from(body)))
        }
        _ => {
            log::debug!("Request admin page failed, Redirect to /login");
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
