use database::models::establish_connection;
use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};

use std::{convert::Infallible, net::SocketAddr};

#[macro_use]
extern crate diesel;

mod database;
mod router;

#[tokio::main]
async fn main() {
    env_logger::init();
    let _ = establish_connection();
    // TODO: parse config file

    if let Err(e) = router::init() {
        eprintln!("Failed to initialize router: {}", e);
        std::process::exit(1);
    }

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let make_service =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(router::handle)) });
    let server = Server::bind(&addr).serve(make_service);
    log::info!("Listening on http://{}", addr);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
