mod server;
mod database;

use std::sync::LazyLock;
use anyhow::{anyhow, Context};
use config::{Config, FileFormat};
use serde::Deserialize;
pub use server::ServerConfig;
use crate::config::database::DatabaseConfig;

static CONFIG:LazyLock<AppConfig> = LazyLock::new(|| AppConfig::load().expect("appconfig load fail"));

#[derive(Debug, Deserialize)]
pub struct AppConfig {
     server: ServerConfig,
     database: DatabaseConfig,
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        Config::builder()
            .add_source(
                config::File::with_name("application")
                    .format(FileFormat::Yaml)
                    .required(true),
            )
            .add_source(
                config::Environment::with_prefix("app")
                    .separator("_")
                    .ignore_empty(true),
            )
            .build()
            .with_context(|| anyhow!("Could not load config file"))?
            .try_deserialize()
            .with_context(|| anyhow!("Could not parse config file"))
    }

    pub fn database(&self) -> &DatabaseConfig {
        &self.database
    }

    pub fn server(&self) -> &ServerConfig {
        &self.server
    }
}

pub fn config() -> &'static AppConfig {
    &CONFIG
}
