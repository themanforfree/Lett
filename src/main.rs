use database::models::establish_connection;
use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};
// use log::info;
use std::{convert::Infallible, net::SocketAddr};

#[macro_use]
extern crate diesel;

mod database;
mod router;
use router::RouteTable;
// use router::RouteTable;

#[tokio::main]
async fn main() {
    env_logger::init();

    let _ = establish_connection();

    if let Err(e) = RouteTable::init() {
        eprintln!("Failed to init route table: {}", e);
        std::process::exit(1);
    }

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_service =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(RouteTable::handle)) });

    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
