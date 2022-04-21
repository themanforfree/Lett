use anyhow::{anyhow, Result};
use hyper::{Body, Request, Response};
use hyper_staticfile::Static;
use matchit::Router;
use once_cell::sync::OnceCell;
use pulldown_cmark::{html, Options, Parser};
use std::{collections::HashMap, convert::Infallible};
use tera::{to_value, Context, Tera, Value};
use time::{format_description as fd, macros::format_description, OffsetDateTime, UtcOffset};

use crate::config::CONFIG;

mod admin;
mod archive;
mod comment;
mod delete;
mod index;
mod login;
mod new;
mod post;
mod search;
mod update;

static ROUTE_TABLE: OnceCell<Router<RouterType>> = OnceCell::new();
static TEMPLATES: OnceCell<Tera> = OnceCell::new();
static ADMIN_TEMPLATES: OnceCell<Tera> = OnceCell::new();
static STATIC_FILES: OnceCell<Static> = OnceCell::new();

enum RouterType {
    Index,
    Archive,
    Search,
    Admin,
    New,
    Delete,
    Login,
    Post,
    Update,
    Comment,
}

pub fn md2html(md: &str) -> String {
    let parser = Parser::new_ext(md, Options::all());
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}

pub fn init() -> Result<()> {
    let cfg = CONFIG.get().unwrap();
    let mut router = Router::new();
    router.insert("/", RouterType::Index)?;
    router.insert("/:year/:month", RouterType::Archive)?;
    router.insert("/:year/:month/", RouterType::Archive)?;
    router.insert("/post/:id", RouterType::Post)?;

    router.insert("/search", RouterType::Search)?;
    router.insert("/admin", RouterType::Admin)?;
    router.insert("/admin/:path", RouterType::Admin)?;

    router.insert("/admin/new", RouterType::New)?;
    router.insert("/admin/delete", RouterType::Delete)?;
    router.insert("/admin/update", RouterType::Update)?;
    router.insert("/login", RouterType::Login)?;
    router.insert("/login/", RouterType::Login)?;
    router.insert("/comment", RouterType::Comment)?;

    ROUTE_TABLE
        .set(router)
        .map_err(|_| anyhow!("Failed to initialize router"))?;

    let files = cfg.application.template_path.clone() + "/**/*.html";

    let mut tera = Tera::new(&files).expect("Failed to compile templates");

    let fmt_tz = format_description!("[offset_hour]:[offset_minute]");
    let offset = UtcOffset::parse(&cfg.application.timezone, fmt_tz).unwrap();
    let timestamp2time = move |args: &HashMap<String, Value>| {
        if let Some(timestamp) = args.get("timestamp") {
            let timestamp = timestamp.as_i64().unwrap();
            let fmt_datetime = fd::parse(&cfg.application.time_format).unwrap();
            let time = OffsetDateTime::from_unix_timestamp(timestamp)
                .unwrap()
                .to_offset(offset)
                .format(&fmt_datetime)
                .unwrap();

            Ok(to_value(time).unwrap())
        } else {
            Err("Some Err".into())
        }
    };
    tera.register_function("timestamp2time", timestamp2time);
    TEMPLATES
        .set(tera)
        .map_err(|_| anyhow!("Failed to initialize tera"))?;
    ADMIN_TEMPLATES
        .set({
            let mut admin_tera = Tera::default();
            admin_tera.add_raw_templates(vec![
                ("layout.html", include_str!("admin_template/layout.html")),
                ("list.html", include_str!("admin_template/list.html")),
                ("new.html", include_str!("admin_template/new.html")),
                ("update.html", include_str!("admin_template/update.html")),
                ("login.html", include_str!("admin_template/login.html")),
            ])?;
            admin_tera.register_function("timestamp2time", timestamp2time);
            admin_tera
        })
        .map_err(|_| anyhow!("Failed to initialize tera"))?;
    STATIC_FILES
        .set(Static::new(
            cfg.application.template_path.clone() + "/static",
        ))
        .map_err(|_| anyhow!("Failed to initialize static files"))?;
    Ok(())
}

async fn merge(req: Request<Body>) -> Option<Response<Body>> {
    let router = ROUTE_TABLE.get().unwrap();
    let path = req.uri().path().to_string();
    if let Ok(matched) = router.at(&path) {
        match matched.value {
            RouterType::Delete => delete::handle(req, matched.params).await,
            RouterType::Comment => comment::handle(req, matched.params).await,
            RouterType::Search => search::handle(req, matched.params).await,
            RouterType::Admin => admin::handle(req, matched.params).await,
            RouterType::Index => index::handle(req, matched.params).await,
            RouterType::Update => update::handle(req, matched.params).await,
            RouterType::New => new::handle(req, matched.params).await,
            RouterType::Login => login::handle(req, matched.params).await,
            RouterType::Archive => archive::handle(req, matched.params).await,
            RouterType::Post => post::handle(req, matched.params).await,
        }
    } else {
        STATIC_FILES.get().unwrap().clone().serve(req).await.ok()
    }
}

pub async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path().to_owned();
    match merge(req).await {
        Some(res) => Ok(res),
        None => {
            log::debug!("Not Found: {}", path);
            let mut context = Context::new();
            let cfg = CONFIG.get().unwrap();
            context.insert("site", &cfg.site);
            let body = TEMPLATES
                .get()
                .unwrap()
                .render("404.html", &context)
                .unwrap();
            Ok(Response::new(hyper::Body::from(body)))
        }
    }
}
