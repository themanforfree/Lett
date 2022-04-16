use crate::config::Config;

use once_cell::sync::OnceCell;
use std::env;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod config;
mod database;
mod router;
mod server;

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

    if let Err(e) = database::init(&config) {
        log::error!("Failed to initialize database: {}", e);
        std::process::exit(1);
    }

    if let Err(e) = router::init(&config) {
        log::error!("Failed to initialize router: {}", e);
        std::process::exit(1);
    }

    if let Err(e) = server::run(&config).await {
        eprintln!("server error: {}", e);
    }
}
