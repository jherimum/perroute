use crate::configuration::env::Environment;
use ::config::{Config, ConfigError};
use secrecy::SecretString;
use serde::Deserialize;
use tap::{Tap, TapFallible};

#[derive(Debug, thiserror::Error)]
pub enum SettingsError {
    #[error("Failed to load settings: {0}")]
    ConfigError(#[from] ConfigError),
}

#[derive(Deserialize, Clone, Debug)]
pub struct Settings {
    pub server: Option<ServerSettings>,
    pub database: Option<DatabaseSettings>,
    pub template_storage: Option<AwsS3TemplateStorageSettings>,
    pub aws: Option<AwsSettings>,
    pub pooling: Option<EventPoolingSettings>,
    pub digester: Option<DigesterSettings>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DigesterSettings {
    pub bucket_name: String,
    pub queue_arn: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct EventPoolingSettings {
    pub interval: u64,
    pub max_events: u64,
    pub topic_arn: String,
    pub publishable_events: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AwsSettings {
    pub dispatch_queue_url: String,
    pub digest_queue_url: String,
    pub event_topic_arn: String,
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

impl Settings {
    pub fn load() -> Result<Self, SettingsError> {
        let env = Environment::which();
        log::info!(
            "Starting to loading configuration from {} environment",
            env
        );
        let settings = Config::builder()
            .add_source(
                config::Environment::default()
                    .prefix("PERROUTE")
                    .separator("__")
                    .list_separator(","),
            )
            .build()
            .tap_err(|e| log::error!("{:?}", e))?;
        Ok(settings
            .try_deserialize::<Self>()
            .tap_err(|e| {
                log::error!("Failed to deserialize settings. Error: {e}")
            })
            .tap(|s| log::debug!("Settings loaded: {s:#?}"))?)
    }
}
