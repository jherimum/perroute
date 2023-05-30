use serde::Deserialize;
use serde_aux::prelude::deserialize_number_from_string;
use std::{fmt::Display, path::PathBuf, str::FromStr};
use tap::TapFallible;

const APP_ENVIRONMENT_ENV_VAR_NAME: &str = "APP_ENVIRONMENT";

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigurationError {
    #[error("{0} is not a supported environment. Use either 'local' or 'production' or 'staging'")]
    EnvironmentParser(String),
}

impl Settings {
    pub fn load() -> Result<Settings, ConfigurationError> {
        tracing::debug!("Starting to loading configuration");
        let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let configuration_directory = base_path.join("configuration");
        let environment = Environment::from_env()
            .tap_err(|e| tracing::warn!("Invalid environment. Setting default environment"))
            .unwrap_or_default();
        let environment_filename = format!("{}.yaml", environment);

        let settings = config::Config::builder()
            .add_source(config::File::from(
                configuration_directory.join("base.yaml"),
            ))
            .add_source(config::File::from(
                configuration_directory.join(environment_filename),
            ))
            .add_source(
                config::Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()
            .unwrap();
        Ok(settings.try_deserialize::<Settings>().unwrap())
    }
}

#[derive(Deserialize, Clone)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

#[derive(Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub pool: PollSettings,
    pub require_ssl: bool,
}

#[derive(Deserialize, Clone)]
pub struct PollSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub max_connection: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub max_lifetime: u64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub idle_timeout: u64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub acquire_timeout: u64,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum Environment {
    #[default]
    Local,
    Production,
    Staging,
}

impl FromStr for Environment {
    type Err = ConfigurationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            "staging" => Ok(Self::Staging),
            other => Err(ConfigurationError::EnvironmentParser(other.into())),
        }
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Environment::Local => write!(f, "local"),
            Environment::Production => write!(f, "production"),
            Environment::Staging => write!(f, "staging"),
        }
    }
}

impl Environment {
    fn from_env() -> Result<Self, ConfigurationError> {
        std::env::var(APP_ENVIRONMENT_ENV_VAR_NAME)
            .map(|e| Environment::from_str(&e))
            .unwrap()
    }
}
