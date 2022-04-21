use anyhow::Result;
use diesel::{Connection, MysqlConnection};
use getopts::Options;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::{env::ArgsOs, fs, net::SocketAddr};

use crate::database::models::{article, comment, session};

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
    pub time_format: String,
    pub template_path: String,
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
        opts.optopt("i", "install", "create tables", "DATABASE_URL");
        opts.optflag("v", "version", "Print the version");
        opts.optflag("h", "help", "Print this help menu");

        let matches = opts.parse(args.skip(1))?;
        if matches.opt_present("help") {
            println!("{}", opts.usage(env!("CARGO_PKG_NAME")));
            std::process::exit(1);
        }

        if matches.opt_present("version") {
            println!("{}", env!("CARGO_PKG_VERSION"));
            std::process::exit(1);
        }

        if let Some(database_url) = matches.opt_str("install") {
            let conn = MysqlConnection::establish(&database_url)?;
            if let Ok(_) = article::read_all(&conn) {
                log::error!("table 'article' already exists");
            } else if let Ok(_) = comment::read_all(&conn) {
                log::error!("table 'comment' already exists");
            } else if let Ok(_) = session::read_all(&conn) {
                log::error!("table 'session' already exists");
            } else {
                embedded_migrations::run(&conn)?;
                log::info!("database installed");
            }
            std::process::exit(1);
        }

        let config_path = matches
            .opt_str("config")
            .unwrap_or_else(|| "config.toml".to_string());

        let config_str = fs::read_to_string(&config_path)?;
        let mut config: Config = toml::from_str(&config_str)?;
        if config.application.template_path.ends_with('/') {
            config.application.template_path.pop();
        }

        CONFIG.set(config).unwrap();
        Ok(())
    }
}
