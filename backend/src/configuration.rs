use config::Config;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use sqlx::{
    ConnectOptions,
    postgres::{PgConnectOptions, PgSslMode},
};

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

#[derive(Deserialize, Clone)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Clone)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: SecretString,
    pub database_name: String,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres:{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        )
    }

    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Allow
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
            .username(&self.username)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        let options = self.without_db().database(&self.database_name);

        options.log_statements(tracing::log::LevelFilter::Trace)
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to Load the environment var");

    let settings = Config::builder()
        .add_source(config::File::with_name("configuration/base"))
        .add_source(config::File::with_name(&format!(
            "configuration/{}",
            environment.as_str()
        )))
        .add_source(config::Environment::with_prefix("APP").separator("__"))
        .build()?;

    settings.try_deserialize::<Settings>()
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!("{} is not a valid env var", other)),
        }
    }
}
