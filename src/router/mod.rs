use anyhow::{anyhow, Result};
use matchit::Router;
use std::future::Future;
use tokio::sync::{OnceCell, RwLock};

mod index;
mod not_found;

type HyperReq = hyper::Request<hyper::Body>;
type HyperRes = hyper::Response<hyper::Body>;

static ROUTE_TABLE: OnceCell<RouteTable> = OnceCell::const_new();

#[async_trait::async_trait]
pub trait HTTPHandler: Send + Sync + 'static {
    async fn handle(&self, req: HyperReq) -> HyperRes;
}

#[async_trait::async_trait]
impl<F: Send + Sync + 'static, Fut> HTTPHandler for F
where
    F: Fn(HyperReq) -> Fut,
    Fut: Future<Output = HyperRes> + Send + 'static,
{
    async fn handle(&self, ctx: HyperReq) -> HyperRes {
        self(ctx).await
    }
}

pub struct RouteTable {
    routes: RwLock<Router<Box<dyn HTTPHandler>>>,
}

impl RouteTable {
    pub fn init() -> Result<()> {
        let mut routes: Router<Box<dyn HTTPHandler>> = Router::new();
        routes.insert("/", Box::new(index::handler))?;
        ROUTE_TABLE
            .set(RouteTable {
                routes: RwLock::new(routes),
            })
            .map_err(|_| anyhow!("Failed to init route table"))?;
        Ok(())
    }

    pub async fn handle(req: HyperReq) -> Result<HyperRes> {
        let route_table = ROUTE_TABLE.get().unwrap();
        let path = req.uri().path();
        let res = if let Ok(handler) = route_table.routes.read().await.at(path) {
            handler.value.handle(req).await
        } else {
            not_found::handler(req).await
        };
        Ok(res)
    }
}
