use crate::database::{
    establish_connection,
    models::comment::{self, NewComment},
};
use hyper::{Body, Request, Response};

pub(crate) async fn handle(req: Request<Body>) -> Option<Response<Body>> {
    log::debug!("Post to Comment");
    let body = hyper::body::to_bytes(req.into_body()).await.ok()?;
    let article = NewComment::from(body);
    match comment::create(&establish_connection(), &article) {
        Ok(u) => Some(Response::new(Body::from(format!(
            "Crate {} comment success",
            u
        )))),
        Err(_) => Some(Response::new(Body::from(format!("Crate comment failed")))),
    }
}
