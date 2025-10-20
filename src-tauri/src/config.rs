use std::time::Duration;

use clap::Parser;
use config::Config;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

#[derive(Debug, Parser)]
#[command()]
struct Cli {
    #[arg(long = "config.path", env = "ENOTE_CONFIG_LOCATION")]
    config_path: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct Configuration {
    pub config: Config,
}

impl Configuration {
    pub fn new(app_handle: &AppHandle) -> Result<Self, Box<dyn std::error::Error>> {
        let config_dir = app_handle
            .path()
            .app_data_dir()
            .expect("无法获取数据目录")
            .join("application.yml");

        let config = Config::builder()
            .add_source(config::File::with_name(
                config_dir.to_str().unwrap_or_default(),
            ))
            .build()
            .unwrap();

        Ok(Self { config })
    }

    pub async fn database_connection(
        &self,
    ) -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
        let url = self.config.get_string("datasource.url")?;
        let max_connections = self
            .config
            .get_int("datasource.max-connections")
            .unwrap_or(100) as u32;
        let min_connections = self
            .config
            .get_int("datasource.min-connections")
            .unwrap_or(1) as u32;
        let connect_timeout = self
            .config
            .get_int("datasource.connect_timeout")
            .unwrap_or(30) as u64;
        let acquire_timeout = self
            .config
            .get_int("datasource.acquire-timeout")
            .unwrap_or(8) as u64;
        let idle_timeout = self.config.get_int("datasource.idle-time").unwrap_or(10) as u64;
        let max_lifetime = self.config.get_int("datasource.max-lifetime").unwrap_or(30) as u64;

        let mut opt = ConnectOptions::new(url);

        opt.max_connections(max_connections)
            .min_connections(min_connections)
            .connect_timeout(Duration::from_secs(connect_timeout))
            .acquire_timeout(Duration::from_secs(acquire_timeout))
            .idle_timeout(Duration::from_secs(idle_timeout))
            .max_lifetime(Duration::from_secs(max_lifetime));

        Ok(Database::connect(opt).await?)
    }
}

pub struct AppState {
    pub configuration: Configuration,
    pub database_connection: DatabaseConnection,
}
