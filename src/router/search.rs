use crate::{
    config::CONFIG,
    database::{establish_connection, models::article},
    router::{md2html, TEMPLATES},
};
use hyper::{Body, Request, Response};
use matchit::Params;
use serde::Deserialize;
use tera::Context;

#[derive(Deserialize)]
struct SearchParams {
    keyword: String,
}

pub async fn handle(req: Request<Body>, _params: Params<'_, '_>) -> Option<Response<Body>> {
    let query = req.uri().query()?;
    let params: SearchParams = serde_urlencoded::from_str(query).ok()?;
    let mut articles =
        article::search(&establish_connection(), &params.keyword).unwrap_or_default();
    for atc in articles.iter_mut() {
        atc.content = md2html(&atc.content);
    }
    log::debug!("Request search page: keyword = {}", params.keyword);
    let mut content = Context::new();

    let cfg = CONFIG.get().unwrap();
    content.insert("site", &cfg.site);
    content.insert("title", "Search");
    content.insert("articles", &articles);

    let body = TEMPLATES
        .get()
        .unwrap()
        .render("list.html", &content)
        .unwrap();
    Some(Response::new(Body::from(body)))
}
