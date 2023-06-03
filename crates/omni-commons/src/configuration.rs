use secrecy::Secret;
use serde::Deserialize;
use serde_aux::prelude::deserialize_number_from_string;
use std::{
    fmt::Debug,
    path::{Path, PathBuf},
    str::FromStr,
};
use strum_macros::EnumString;
use tap::{Tap, TapFallible};

const RUN_ENV: &str = "RUN_ENV";
const CONFIG_DIR: &str = "CONFIG_DIR";
const BASE_CONFIG_FILENAME: &str = "base.yaml";

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

pub fn load_configuration<'de, S: Deserialize<'de> + Clone + Debug>() -> anyhow::Result<S> {
    let environment = Environment::which();
    tracing::info!(
        "Starting to loading configuration from {} environment",
        environment
    );
    let config_dir = config_path();
    let environment_filename = format!("{}.yaml", environment).to_lowercase();
    let settings = config::Config::builder()
        .add_source(config::File::from(config_dir.join(BASE_CONFIG_FILENAME)))
        .add_source(config::File::from(config_dir.join(environment_filename)))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()
        .tap_err(|e| tracing::error!("{:?}", e))?;
    settings
        .try_deserialize::<S>()
        .tap_err(|e| tracing::error!("Faile to deserialize settings. Error: {e}"))
        .map_err(anyhow::Error::from)
        .tap(|s| tracing::debug!("Settings loaded: {s:#?}"))
}

/// Config path.
pub fn config_path() -> PathBuf {
    let mut config_path = PathBuf::new();
    let config_directory = std::env::var(CONFIG_DIR).unwrap_or_else(|_| "configuration".into());
    config_path.push(workspace_path());
    config_path.push(config_directory);
    config_path
}

pub fn workspace_path() -> PathBuf {
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_dir);
        path.pop();
        path.pop();
        path
    } else {
        PathBuf::from(".")
    }
}

#[derive(Debug, Default, PartialEq, Eq, EnumString, strum::Display, Clone, Copy)]
#[strum(ascii_case_insensitive)]
pub enum Environment {
    #[default]
    Development,
    Production,
    Sandbox,
}

impl Environment {
    fn which() -> Self {
        std::env::var(RUN_ENV)
            .tap_err(|e| tracing::warn!("Failed to fetch [{}] from environment: {e:?}", RUN_ENV))
            .map_or_else(
                |_| Default::default(),
                |v| {
                    v.parse()
                        .tap_err(|e| tracing::warn!("Failed to parse {} to environment: {e:?}", v))
                        .unwrap_or(Default::default())
                },
            )
    }
}
