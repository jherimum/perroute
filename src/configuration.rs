use anyhow::Result;
use secrecy::Secret;
use serde::Deserialize;
use serde_aux::prelude::deserialize_number_from_string;
use std::{
    net::{AddrParseError, SocketAddr},
    path::PathBuf,
    str::FromStr,
};
use strum_macros::EnumString;
use tap::{Tap, TapFallible};

const APP_ENVIRONMENT_ENV_VAR_NAME: &str = "APP_ENVIRONMENT";

impl TryFrom<Settings> for SocketAddr {
    type Error = AddrParseError;

    fn try_from(value: Settings) -> Result<Self, Self::Error> {
        SocketAddr::from_str(&format!(
            "{}:{}",
            value.application.host, value.application.port
        ))
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub pool: PollSettings,
    pub require_ssl: bool,
}

#[derive(Deserialize, Clone, Debug)]
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

impl Settings {
    pub fn load() -> Result<Settings> {
        let environment = Environment::from_env()
            .tap_err(|e| {
                tracing::error!("Failed to resolve application environment: {e:?}");
                tracing::warn!("Setting default environment: {}", Environment::default())
            })
            .unwrap_or_default();
        tracing::info!(
            "Starting to loading configuration from {} environment",
            environment
        );
        let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let configuration_directory = base_path.join("configuration");
        let environment_filename = format!("{}.yaml", environment);
        let settings = config::Config::builder()
            .add_source(config::File::from(
                configuration_directory.join("base.yaml"),
            ))
            .add_source(config::File::from(
                configuration_directory.join(environment_filename),
            ))
            .add_source(
                config::Environment::with_prefix("OMNI")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()
            .unwrap();
        settings
            .try_deserialize::<Settings>()
            .tap_err(|e| tracing::error!("Faile to deserialize settings. Error: {e}"))
            .map_err(anyhow::Error::from)
            .tap(|s| tracing::debug!("Settings loaded: {s:#?}"))
    }
}

#[derive(Debug, Default, PartialEq, Eq, EnumString, strum::Display)]
#[strum(ascii_case_insensitive)]
enum Environment {
    #[default]
    Local,
    Production,
    Staging,
}

impl Environment {
    fn from_env() -> anyhow::Result<Self> {
        std::env::var(APP_ENVIRONMENT_ENV_VAR_NAME)
            .tap_err(|e| {
                tracing::error!(
                    "Failed to fetch [{}] from environment: {e:?}",
                    APP_ENVIRONMENT_ENV_VAR_NAME
                )
            })
            .map_err(anyhow::Error::from)
            .map(|e| {
                Environment::from_str(&e)
                    .tap_err(|error| {
                        tracing::error!("Failed to parse '{}' into Environment: {error:?}", &e,)
                    })
                    .map_err(anyhow::Error::from)
            })?
    }
}
