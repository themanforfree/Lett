use anyhow::{anyhow, Result};
use hyper::{Body, Method, Request, Response};
use matchit::Router;
use once_cell::sync::OnceCell;
use pulldown_cmark::{html, Options, Parser};
use std::{collections::HashMap, convert::Infallible};
use tera::{to_value, Context, Tera, Value};
use time::{macros::format_description, OffsetDateTime, UtcOffset};

use crate::{config::Site, TIMEZONE};

mod admin;
mod archive;
mod delete;
mod index;
mod login;
mod new;
mod post;
mod search;
mod static_files;

static ROUTE_TABLE: OnceCell<Router<RouterType>> = OnceCell::new();
static TEMPLATES: OnceCell<Tera> = OnceCell::new();
static SITE: OnceCell<Site> = OnceCell::new();
enum RouterType {
    Index,
    Archive,
    Search,
    Admin,
    New,
    Delete,
    Login,
    Post,
}

pub(crate) fn md2html(md: &str) -> String {
    let parser = Parser::new_ext(md, Options::all());
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}

pub(crate) fn init(cfg: Site) -> Result<()> {
    let mut router = Router::new();
    router.insert("/", RouterType::Index)?;
    router.insert("/:year/:month", RouterType::Archive)?;
    router.insert("/:year/:month/", RouterType::Archive)?;
    router.insert("/post/:id", RouterType::Post)?;
    router.insert("/post/:id/", RouterType::Post)?;

    router.insert("/search", RouterType::Search)?;
    router.insert("/admin", RouterType::Admin)?;
    router.insert("/admin/", RouterType::Admin)?;

    router.insert("/new", RouterType::New)?;
    router.insert("/delete", RouterType::Delete)?;
    router.insert("/login", RouterType::Login)?;
    router.insert("/login/", RouterType::Login)?;

    ROUTE_TABLE
        .set(router)
        .map_err(|_| anyhow!("Failed to initialize router"))?;

    let mut tera = Tera::new("templates/**/*.html").expect("Failed to compile templates");
    tera.register_function("url_for", |args: &HashMap<String, Value>| {
        if let Some(id) = args.get("id") {
            Ok(to_value(format!("/post/{}", &id)).unwrap())
        } else {
            Err("Some Err".into())
        }
    });

    let fmt_tz = format_description!("[offset_hour]:[offset_minute]");
    let timezone = TIMEZONE.get().unwrap();
    let offset = UtcOffset::parse(timezone, fmt_tz).unwrap();
    tera.register_function("timestamp2time", move |args: &HashMap<String, Value>| {
        if let Some(timestamp) = args.get("timestamp") {
            let timestamp = timestamp.as_i64().unwrap();
            let fmt_datetime = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
            let time = OffsetDateTime::from_unix_timestamp(timestamp)
                .unwrap()
                .to_offset(offset)
                .format(&fmt_datetime)
                .unwrap();

            Ok(to_value(time).unwrap())
        } else {
            Err("Some Err".into())
        }
    });
    TEMPLATES.set(tera).unwrap();
    SITE.set(cfg).unwrap();
    Ok(())
}

async fn merge(req: Request<Body>) -> Option<Response<Body>> {
    let router = ROUTE_TABLE.get().unwrap();
    let path = req.uri().path();
    if let Ok(matched) = router.at(path) {
        match (req.method(), matched.value) {
            (&Method::POST, RouterType::New) => new::handle(req).await,
            (&Method::POST, RouterType::Delete) => delete::handle(req).await,
            (&Method::GET, RouterType::Search) => search::handle(req).await,
            (&Method::GET, RouterType::Admin) => admin::handle(req).await,
            (&Method::GET, RouterType::Index) => index::handle(req).await,
            (_, RouterType::Login) => login::handle(req).await,
            (&Method::GET, RouterType::Archive) => {
                let year = matched.params.get("year")?.to_owned();
                let month = matched.params.get("month")?.to_owned();
                archive::handle(req, &year, &month).await
            }
            (&Method::GET, RouterType::Post) => {
                let id = matched.params.get("id")?.to_owned();
                post::handle(req, &id).await
            }
            _ => None,
        }
    } else {
        static_files::handle(req).await
    }
}

pub(crate) async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path().to_owned();
    match merge(req).await {
        Some(res) => Ok(res),
        None => {
            log::debug!("Not Found: {}", path);
            let mut context = Context::new();
            context.insert("site", &SITE.get().unwrap());
            let body = TEMPLATES
                .get()
                .unwrap()
                .render("404.html", &context)
                .unwrap();
            Ok(Response::new(hyper::Body::from(body)))
        }
    }
}
