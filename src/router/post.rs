use crate::{
    config::CONFIG,
    database::{
        establish_connection,
        models::{article, comment},
    },
    router::{md2html, TEMPLATES},
};
use hyper::{Body, Request, Response};
use matchit::Params;
use tera::Context;

pub async fn handle(_req: Request<Body>, params: Params<'_, '_>) -> Option<Response<Body>> {
    let id = params.get("id")?.parse().ok()?;
    log::debug!("Request post: aid = {}", id);
    let mut atc = article::read_by_id(&establish_connection(), id).ok()?;
    atc.content = md2html(&atc.content);
    let cmt = comment::read_by_aid(&establish_connection(), id).ok()?;

    let cfg = CONFIG.get().unwrap();
    let mut context = Context::new();
    context.insert("site", &cfg.site);
    context.insert("article", &atc);
    context.insert("comments", &cmt);
    let body = TEMPLATES
        .get()
        .unwrap()
        .render("post.html", &context)
        .ok()?;
    Some(Response::new(Body::from(body)))
}
