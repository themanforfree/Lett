use anyhow::Result;
use dotenv::dotenv;
use getopts::Options;
use serde::{Deserialize, Serialize};
use std::{
    env::{self, ArgsOs},
    fs,
    net::SocketAddr,
};

use crate::TIMEZONE;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub application: Application,
    pub database: Database,
    pub site: Site,
}

#[derive(Deserialize, Debug)]
pub struct Application {
    pub listen: SocketAddr,
    pub timezone: String,
    pub tls: bool,
    pub certs: String,
    pub key: String,
}

#[derive(Deserialize, Debug)]
pub struct Database {
    pub url: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Site {
    pub name: String,
    pub url: String,
    pub description: String,
}

impl Config {
    pub fn parse(args: ArgsOs) -> Result<Config> {
        dotenv().ok();

        let mut opts = Options::new();
        opts.optopt("c", "config", "read config from file", "CONFIG_PATH");

        let matches = opts.parse(args.skip(1))?;

        let config_path = matches
            .opt_str("config")
            .unwrap_or_else(|| "config.toml".to_string());

        let config_str = fs::read_to_string(&config_path)?;
        let mut config: Config = toml::from_str(&config_str)?;

        if let Ok(url) = env::var("DATABASE_URL") {
            config.database.url = url;
        }
        TIMEZONE.set(config.application.timezone.clone()).unwrap();
        Ok(config)
    }
}
