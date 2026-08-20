use config::{self, Config, ConfigBuilder};
use serde::Deserialize;

use crate::configuration::Environment::Local;

#[derive(Deserialize)]
pub struct Settings {
 pub application: ApplicationSettings,
 pub database: DatabaseSettings
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
 pub host: String,
 pub port: u16
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
 pub host: String,
 pub port: u16,
 pub username: String,
 pub password: String,
 pub database_name: String,
 pub require_ssl: bool
}


pub fn get_configuration () -> Result<Settings, config::ConfigError> {

 let environment: Environment = std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "local".into()).try_into().expect("Failed to Load the environment var");

 let settings = Config::builder().add_source(config::File::with_name("configuration/base")).add_source(config::File::with_name(&format!("configuration/{}", environment.as_str()))).add_source(config::Environment::with_prefix("APP").separator("__")).build()?;

 settings.try_deserialize::<Settings>()
}

pub enum Environment {
    Local,
    Production
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
     match self {
      Self::Local => "local",
      Self::Production => "production"
     }
    }
}

impl TryFrom<String> for Environment {
 type Error = String;
 fn try_from(value: String) -> Result<Self, Self::Error> {
     match value.to_lowercase().as_str() {
      "local" => Ok(Self::Local),
      "production" => Ok(Self::Production),
      other => Err(format!("{} is not a valid env var", other))
     }
 }
}