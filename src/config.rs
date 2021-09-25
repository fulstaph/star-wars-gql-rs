//! src/config.rs

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: Database,
    pub port: u16,
    pub host: String,
}

impl Settings {
    pub fn addr(&self) -> String {
        self.host.to_owned() + ":" + &self.port.to_string()
    }
}

#[derive(serde::Deserialize)]
pub struct Database {
    pub url: String,
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    dotenv::dotenv().ok();

    let app_name = std::env::var("APP_NAME").unwrap_or_else(|_| "APP".into());

    let mut settings = config::Config::default();

    settings.merge(config::Environment::with_prefix(app_name.as_str()).separator("_"))?;

    settings.try_into()
}
