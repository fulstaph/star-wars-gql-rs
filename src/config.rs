//! src/config.rs

use std::collections::HashMap;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: Database,
    pub port: u16
}

impl Settings {
    pub fn addr(&self) -> String {
        String::from("127.0.0.1:".to_owned() + &self.port.to_string())
    }
}

#[derive(serde::Deserialize)]
pub struct Database {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub name: String,
}

impl Database {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password,
            self.host,
            self.port,
            self.name
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password,
            self.host,
            self.port
        )
    }
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    dotenv::dotenv().ok();

    let app_name = std::env::var("APP_NAME").unwrap_or_else(|_| "APP".into());

    let mut settings = config::Config::default();

    // settings.merge(config::File::with_name("config"))?;

    settings.merge(config::Environment::with_prefix(app_name.as_str()).separator("_"))?;

    // settings.merge(DotEnv)?;

    settings.try_into()
}
