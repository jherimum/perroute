use anyhow::Result;
use perroute_commons::configuration::settings::DatabaseSettings;
use secrecy::ExposeSecret;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    Connection as SqlxConn, PgConnection, PgPool,
};
use std::time::Duration;
use tap::TapFallible;

pub type ConnectionPool = PgPool;
pub type Connection = PgConnection;

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!();

pub struct ConnectionManager;

impl ConnectionManager {
    pub async fn build_pool(settings: &DatabaseSettings) -> Result<ConnectionPool> {
        let options = Self::connection_options(settings);
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
        if settings.migration.enabled {
            Self::migrate(&pool).await?;
        }

        Ok(pool)
    }

    pub async fn migrate(pool: &PgPool) -> Result<()> {
        tracing::info!("Migration started");
        sqlx::migrate!()
            .run(pool)
            .await
            .tap_err(|e| tracing::error!("Failed to run migrations: {e}"))?;
        tracing::info!("Migrations finished");

        Ok(())
    }

    pub async fn new_connection(
        database_settings: &DatabaseSettings,
    ) -> Result<PgConnection, sqlx::Error> {
        let options = Self::connection_options(database_settings);
        PgConnection::connect_with(&options).await
    }

    pub fn connection_options(database_settings: &DatabaseSettings) -> PgConnectOptions {
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
            .database(&database_settings.database_name)
    }
}
