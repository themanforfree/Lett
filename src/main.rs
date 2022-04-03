use database::models::establish_connection;
use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};
use lazy_static::lazy_static;
use std::{convert::Infallible, net::SocketAddr};
use tera::Tera;

#[macro_use]
extern crate diesel;

mod database;
mod router;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Parsing error(s): {}", e);
                std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![]);
        tera
    };
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let _ = establish_connection();

    if let Err(e) = router::init() {
        eprintln!("Failed to initialize router: {}", e);
        std::process::exit(1);
    }

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let make_service =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(router::handle)) });
    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
