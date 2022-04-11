use crate::{
    database::{establish_connection, models::article},
    router::{md2html, TEMPLATES},
};
use hyper::{Body, Request, Response};
use tera::Context;

pub(crate) async fn handle(_req: Request<Body>, id: &str) -> Option<Response<Body>> {
    let id = id.parse().ok()?;
    log::debug!("Request post: aid = {}", id);
    let mut atc = article::read_by_id(&establish_connection(), id).ok()?;
    atc.content = md2html(&atc.content);
    let mut context = Context::new();
    context.insert("article", &atc);
    let body = TEMPLATES
        .get()
        .unwrap()
        .render("post.html", &context)
        .ok()?;
    Some(Response::new(Body::from(body)))
}
