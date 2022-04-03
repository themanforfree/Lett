use std::convert::Infallible;

use anyhow::{anyhow, Result};
use matchit::Router;
use tokio::sync::{OnceCell, RwLock};

mod admin;
mod archive;
mod index;
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
    // TODO: Add more routes here
}

pub fn init() -> Result<()> {
    let mut router = Router::new();
    router.insert("/", RouterType::Index)?;
    router.insert("/:year/:month", RouterType::Archive)?;
    router.insert("/search", RouterType::Search)?;
    router.insert("/admin", RouterType::Admin)?;
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
        match matched.value {
            RouterType::Index => return Ok(index::handle(req).await),
            RouterType::Archive => {
                let year = match matched.params.get("year").and_then(|s| s.parse().ok()) {
                    Some(year) => year,
                    None => return Ok(not_found::handle(req).await),
                };

                let month = match matched
                    .params
                    .get("month")
                    .and_then(|month| month.parse().ok())
                {
                    Some(month) => month,
                    None => return Ok(not_found::handle(req).await),
                };
                return Ok(archive::handle(req, year, month).await);
            }
            RouterType::Search => return Ok(search::handle(req).await),
            RouterType::Admin => return Ok(admin::handle(req).await),
            // TODO: Add more routes here
        }
    }
    Ok(not_found::handle(req).await)
}
