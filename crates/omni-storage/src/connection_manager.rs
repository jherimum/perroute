use anyhow::Result;
use omni_commons::configuration::settings::DatabaseSettings;
use secrecy::ExposeSecret;
use sqlx::{
    migrate::Migrator,
    pool,
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    Acquire, Connection as SqlxConn, PgConnection, PgPool, Postgres,
};
use std::{path::PathBuf, time::Duration};
use tap::TapFallible;

pub type ConnectionPool = PgPool;
pub type Connection = PgConnection;

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
        database_name: Option<String>,
        seed: Vec<PathBuf>,
    ) -> Result<ConnectionPool> {
        let options = Self::connection_options(settings, database_name);
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
            MigrantionMode::Run => Self::migrate(&pool, seed).await?,
            _ => tracing::debug!("Migration skiped"),
        };

        Ok(pool)
    }

    pub async fn migrate(pool: &PgPool, seed: Vec<PathBuf>) -> Result<()> {
        tracing::info!("Migration started");
        sqlx::migrate!()
            .run(pool)
            .await
            .tap_err(|e| tracing::error!("Failed to run migrations: {e}"))?;
        tracing::info!("Migrations finished");

        tracing::info!("Seeding started");
        for path in seed {
            Migrator::new(path).await?.run(pool).await?;
        }

        Ok(())
    }

    pub async fn new_connection(
        database_settings: &DatabaseSettings,
        database_name: Option<String>,
    ) -> Result<PgConnection, sqlx::Error> {
        let options = Self::connection_options(database_settings, database_name);
        PgConnection::connect_with(&options).await
    }

    fn connection_options(
        database_settings: &DatabaseSettings,
        database_name: Option<String>,
    ) -> PgConnectOptions {
        let ssl_mode = if database_settings.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        let options = PgConnectOptions::new()
            .host(&database_settings.host)
            .username(&database_settings.username)
            .password(database_settings.password.expose_secret())
            .port(database_settings.port)
            .ssl_mode(ssl_mode);

        match database_name {
            Some(db_name) => options.database(&db_name),
            None => options,
        }
    }
}
