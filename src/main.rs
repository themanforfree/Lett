use crate::config::Config;
use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};
use once_cell::sync::OnceCell;
use std::{convert::Infallible, env};

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod config;
mod database;
mod router;

static TIMEZONE: OnceCell<String> = OnceCell::new();

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = env::args_os();
    let config = match Config::parse(args) {
        Ok(config) => config,
        Err(e) => {
            log::error!("Failed to parse config: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = database::init(config.database) {
        log::error!("Failed to initialize database: {}", e);
        std::process::exit(1);
    }

    if let Err(e) = router::init(config.site) {
        log::error!("Failed to initialize router: {}", e);
        std::process::exit(1);
    }

    let addr = config.application.listen;
    let make_service =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(router::handle)) });

    let server = Server::bind(&addr).serve(make_service);
    log::info!("Listening on http://{}", addr);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
