use crate::{
    config::CONFIG,
    database::{establish_connection, models::article},
    router::{md2html, TEMPLATES},
};
use hyper::{Body, Request, Response};

use matchit::Params;
use tera::Context;

pub async fn handle(_req: Request<Body>, _params: Params<'_, '_>) -> Option<Response<Body>> {
    log::debug!("Request index page");
    let mut articles = article::read_published(&establish_connection()).unwrap_or_default();
    for atc in articles.iter_mut() {
        atc.content = md2html(&atc.content);
    }
    let cfg = CONFIG.get().unwrap();
    let mut content = Context::new();
    content.insert("site", &cfg.site);
    content.insert("title", &cfg.site.name);
    content.insert("articles", &articles);

    let body = TEMPLATES
        .get()
        .unwrap()
        .render("list.html", &content)
        .unwrap();
    Some(Response::new(Body::from(body)))
}
