use anyhow::Result;
use diesel::{Connection, MysqlConnection};
use getopts::Options;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::{env::ArgsOs, fs, net::SocketAddr};

pub static CONFIG: OnceCell<Config> = OnceCell::new();

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

embed_migrations!();

impl Config {
    pub fn parse(args: ArgsOs) -> Result<()> {
        let mut opts = Options::new();
        opts.optopt("c", "config", "read config from file", "CONFIG_PATH");
        opts.optflag("", "install", "create database");
        let matches = opts.parse(args.skip(1))?;

        let config_path = matches
            .opt_str("config")
            .unwrap_or_else(|| "config.toml".to_string());

        let config_str = fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&config_str)?;

        if matches.opt_present("install") {
            let conn = MysqlConnection::establish(&config.database.url)?;
            embedded_migrations::run(&conn)?;
            println!("Database initialized");
            std::process::exit(1);
        }

        CONFIG.set(config).unwrap();
        Ok(())
    }
}
