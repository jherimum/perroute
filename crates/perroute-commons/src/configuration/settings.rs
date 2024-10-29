use crate::configuration::env::Environment;
use ::config::{Config, ConfigError, File};
use secrecy::SecretString;
use serde::Deserialize;
use std::path::PathBuf;
use tap::{Tap, TapFallible};

const CONFIG_DIR_KEY: &str = "CONFIG_DIR";
const CARGO_MANIFEST_DIR_KEY: &str = "CARGO_MANIFEST_DIR";
const DEFAULT_CONFIG_FOLDER: &str = "configuration";
const BASE_CONFIG_FILENAME: &str = "base.yaml";

#[derive(Debug, thiserror::Error)]
pub enum SettingsError {
    #[error("Failed to load settings: {0}")]
    ConfigError(#[from] ConfigError),
}

#[derive(Deserialize, Clone, Debug)]
pub struct Settings {
    pub server: ServerSettings,
    pub database: DatabaseSettings,
    pub rabbitmq: Option<RabbitMqSettings>,
    pub template_storage: AwsS3TemplateStorageSettings,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AwsS3TemplateStorageSettings {
    pub bucket_name: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ServerSettings {
    pub port: u16,
    pub host: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretString,
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub pool: PollSettings,
    pub require_ssl: bool,
    pub migration: MigrationSettings,
}

#[derive(Deserialize, Clone, Debug)]
pub struct MigrationSettings {
    pub enabled: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct PollSettings {
    pub max_connection: u32,
    pub max_lifetime: u64,
    pub idle_timeout: u64,
    pub acquire_timeout: u64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RabbitMqSettings {
    pub uri: String,
}
fn config_dir() -> PathBuf {
    std::env::var(CONFIG_DIR_KEY).map_or_else(
        |_| {
            std::env::var(CARGO_MANIFEST_DIR_KEY)
                .map_or_else(
                    |_| std::env::current_dir().unwrap(),
                    std::path::PathBuf::from,
                )
                .join(DEFAULT_CONFIG_FOLDER)
        },
        std::path::PathBuf::from,
    )
}

impl Settings {
    pub fn load() -> Result<Self, SettingsError> {
        let env = Environment::which();
        log::info!("Starting to loading configuration from {} environment", env);
        let config_dir = config_dir();
        let environment_filename = format!("{}.yaml", env).to_lowercase();
        let settings = Config::builder()
            .add_source(File::from(config_dir.join(BASE_CONFIG_FILENAME)))
            .add_source(File::from(config_dir.join(environment_filename)))
            .add_source(
                config::Environment::default()
                    .prefix("APP")
                    .prefix_separator("__")
                    .separator("_"),
            )
            .build()
            .tap_err(|e| log::error!("{:?}", e))?;
        Ok(settings
            .try_deserialize::<Self>()
            .tap_err(|e| log::error!("Failed to deserialize settings. Error: {e}"))
            .tap(|s| log::debug!("Settings loaded: {s:#?}"))?)
    }
}
