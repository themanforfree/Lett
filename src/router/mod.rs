use std::convert::Infallible;

use anyhow::{anyhow, Result};
use hyper::Method;
use matchit::Router;
use tokio::sync::{OnceCell, RwLock};

mod admin;
mod archive;
mod index;
mod new;
mod not_found;
mod search;

type HyperReq = hyper::Request<hyper::Body>;
type HyperRes = hyper::Response<hyper::Body>;

static ROUTE_TABLE: OnceCell<RwLock<Router<RouterType>>> = OnceCell::const_new();

enum RouterType {
    Index,
    Archive,
    Search,
    Admin,
    New,
    // TODO: Add more routes here
}

pub fn init() -> Result<()> {
    let mut router = Router::new();
    router.insert("/", RouterType::Index)?;
    router.insert("/:year/:month", RouterType::Archive)?;
    router.insert("/search", RouterType::Search)?;
    router.insert("/admin", RouterType::Admin)?;
    router.insert("/new", RouterType::New)?;
    // TODO: Add more routes here
    ROUTE_TABLE
        .set(RwLock::new(router))
        .map_err(|_| anyhow!("Failed to initialize router"))?;
    Ok(())
}

pub async fn handle(req: HyperReq) -> Result<HyperRes, Infallible> {
    let router = ROUTE_TABLE.get().unwrap().read().await;
    let path = req.uri().path();
    if let Ok(matched) = router.at(path) {
        match (req.method(), matched.value) {
            (&Method::GET, RouterType::Archive) => {
                let year = match matched.params.get("year").and_then(|y| y.parse().ok()) {
                    Some(year) => year,
                    None => return Ok(not_found::handle(req).await),
                };
                let month = match matched.params.get("month").and_then(|m| m.parse().ok()) {
                    Some(month) => month,
                    None => return Ok(not_found::handle(req).await),
                };
                return Ok(archive::handle(req, year, month).await);
            }

            (&Method::GET, RouterType::Index) => return Ok(index::handle(req).await),

            (&Method::GET, RouterType::Search) => return Ok(search::handle(req).await),

            (&Method::GET, RouterType::Admin) => return Ok(admin::handle(req).await),

            (&Method::POST, RouterType::New) => return Ok(new::handle(req).await),

            _ => return Ok(not_found::handle(req).await),
        }
    }
    Ok(not_found::handle(req).await)
}
