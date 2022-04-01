use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use log::info;
use std::{convert::Infallible, net::SocketAddr};

#[macro_use]
extern crate diesel;

mod database;

async fn handle(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let res = Response::new(Body::from("Hello World!"));
    info!("{:#?}", _req);
    info!("{:#?}", res);

    Ok(res)
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
