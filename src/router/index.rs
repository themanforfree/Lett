use crate::{
    database::{establish_connection, models::article},
    router::{md2html, SITE, TEMPLATES},
};
use hyper::{Body, Request, Response};

use tera::Context;

pub(crate) async fn handle(_req: Request<Body>) -> Option<Response<Body>> {
    log::debug!("Request index page");
    let mut articles = article::read_all(&establish_connection()).unwrap_or_default();
    for atc in articles.iter_mut() {
        atc.content = md2html(&atc.content);
    }
    let site = SITE.get().unwrap();
    let mut content = Context::new();
    content.insert("site", &site);
    content.insert("title", &site.name);
    content.insert("articles", &articles);

    let body = TEMPLATES
        .get()
        .unwrap()
        .render("list.html", &content)
        .unwrap();
    Some(Response::new(Body::from(body)))
}
