use anyhow::{anyhow, Result};
use hyper::{Body, Method, Request, Response};
use matchit::Router;
use once_cell::sync::OnceCell;
use std::convert::Infallible;

mod admin;
mod archive;
mod delete;
mod index;
mod login;
mod new;
mod search;

static ROUTE_TABLE: OnceCell<Router<RouterType>> = OnceCell::new();

enum RouterType {
    Index,
    Archive,
    Search,
    Admin,
    New,
    Delete,
    Login,
    // TODO: Add more routes here
}

pub(crate) fn init() -> Result<()> {
    let mut router = Router::new();
    router.insert("/", RouterType::Index)?;
    router.insert("/:year/:month", RouterType::Archive)?;
    router.insert("/:year/:month/", RouterType::Archive)?;

    router.insert("/search", RouterType::Search)?;
    router.insert("/admin", RouterType::Admin)?;
    router.insert("/admin/", RouterType::Admin)?;

    router.insert("/new", RouterType::New)?;
    router.insert("/delete", RouterType::Delete)?;
    router.insert("/login", RouterType::Login)?;
    router.insert("/login/", RouterType::Login)?;
    // TODO: Add more routes here
    ROUTE_TABLE
        .set(router)
        .map_err(|_| anyhow!("Failed to initialize router"))?;
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
            _ => None,
        }
    } else {
        None
    }
}

pub(crate) async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path().to_owned();
    match merge(req).await {
        Some(res) => Ok(res),
        None => {
            log::debug!("Not Found: {}", path);
            Ok(Response::new(hyper::Body::from("Not Found")))
        }
    }
}
