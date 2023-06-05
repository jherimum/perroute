use anyhow::Result;
use omni_commons::configuration::settings::DatabaseSettings;
use secrecy::ExposeSecret;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    Connection, PgConnection, PgPool,
};
use std::time::Duration;
use tap::TapFallible;

pub type OmniMessageConnectionPool = PgPool;

pub struct OmniMessageConnectionManager;

pub enum MigrantionMode {
    Run,
    Skip,
}

pub enum ConnectionBuild {
    WithDatabase,
    WithoutDatabase,
}

impl OmniMessageConnectionManager {
    pub async fn new_pool(
        settings: &DatabaseSettings,
        migration_mode: MigrantionMode,
        connection_build: ConnectionBuild,
    ) -> Result<OmniMessageConnectionPool> {
        let options = Self::connection_options(settings, connection_build);
        let pool = PgPoolOptions::new()
            .max_connections(settings.pool.max_connection)
            .max_lifetime(Some(Duration::from_secs(settings.pool.max_lifetime)))
            .idle_timeout(Some(Duration::from_secs(settings.pool.idle_timeout)))
            .acquire_timeout(Duration::from_secs(settings.pool.acquire_timeout))
            .acquire_timeout(Duration::from_secs(1))
            .connect_with(options.clone())
            .await
            .tap_err(|e| {
                tracing::error!(
                    "Failed to initialize database with options:{:?}. {e}",
                    &options
                )
            })?;

        match migration_mode {
            MigrantionMode::Run => {
                tracing::info!("Migration started");
                sqlx::migrate!()
                    .run(&pool)
                    .await
                    .tap_err(|e| tracing::error!("Failed to run migrations: {e}"))?;
                tracing::info!("Migrations finished");
            }
            _ => tracing::debug!("Migration skiped"),
        };

        Ok(pool)
    }

    pub async fn new_connection(
        database_settings: &DatabaseSettings,
        connection_build: ConnectionBuild,
    ) -> Result<PgConnection, sqlx::Error> {
        let options = Self::connection_options(database_settings, connection_build);
        PgConnection::connect_with(&options).await
    }

    pub fn connection_options(
        database_settings: &DatabaseSettings,
        connection_build: ConnectionBuild,
    ) -> PgConnectOptions {
        match connection_build {
            ConnectionBuild::WithDatabase => Self::with_db(database_settings),
            ConnectionBuild::WithoutDatabase => Self::without_db(database_settings),
        }
    }

    pub fn with_db(database_settings: &DatabaseSettings) -> PgConnectOptions {
        Self::without_db(database_settings).database(&database_settings.database_name)
    }

    pub fn without_db(database_settings: &DatabaseSettings) -> PgConnectOptions {
        let ssl_mode = if database_settings.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&database_settings.host)
            .username(&database_settings.username)
            .password(database_settings.password.expose_secret())
            .port(database_settings.port)
            .ssl_mode(ssl_mode)
    }
}
