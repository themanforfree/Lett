use anyhow::{anyhow, bail, Result};
use matchit::{MatchError, Router};
use tokio::sync::{OnceCell, RwLock};

mod index;
mod not_found;

type HyperReq = hyper::Request<hyper::Body>;
type HyperRes = hyper::Response<hyper::Body>;

static ROUTE_TABLE: OnceCell<RwLock<Router<RouterType>>> = OnceCell::const_new();

enum RouterType {
    Index,
    // TODO: Add more routes here
}

pub fn init() -> Result<()> {
    let mut router = Router::new();
    router.insert("/", RouterType::Index)?;
    // TODO: Add more routes here
    ROUTE_TABLE
        .set(RwLock::new(router))
        .map_err(|_| anyhow!("Failed to initialize router"))?;
    Ok(())
}

pub async fn handle(req: HyperReq) -> Result<HyperRes> {
    let router = ROUTE_TABLE
        .get()
        .ok_or_else(|| anyhow!("Router not initialized"))?
        .read()
        .await;
    let path = req.uri().path();
    let res = match router.at(path) {
        Ok(matched) => match matched.value {
            RouterType::Index => index::handler(req).await,
            // TODO: Add more cases
        },
        Err(e) if e == MatchError::NotFound => not_found::handler(req).await,
        Err(e) => bail!("{}", e),
    };

    Ok(res)
}
