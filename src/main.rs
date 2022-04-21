use std::env;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod config;
mod database;
mod router;
mod server;

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = env::args_os();

    if let Err(e) = config::Config::parse(args) {
        log::error!("Failed to parse config: {}", e);
        std::process::exit(1);
    }

    if let Err(e) = database::init() {
        log::error!("Failed to initialize database: {}", e);
        std::process::exit(1);
    }

    if let Err(e) = router::init() {
        log::error!("Failed to initialize router: {}", e);
        std::process::exit(1);
    }

    if let Err(e) = server::run().await {
        eprintln!("server error: {}", e);
    }
}
