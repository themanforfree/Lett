use hyper::{Body, Request, Response};
use std::path::Path;
use tokio::fs::read;

pub(crate) async fn handle(req: Request<Body>) -> Option<Response<Body>> {
    let base_path = Path::new("files");
    let path = base_path.join(&req.uri().path()[1..]);
    let file = read(&path).await.ok()?;
    Some(Response::new(Body::from(file)))
}
