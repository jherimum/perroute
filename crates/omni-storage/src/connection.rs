use anyhow::Result;
use omni_commons::configuration::DatabaseSettings;
use secrecy::ExposeSecret;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    PgPool,
};
use std::time::Duration;

pub fn build_pool(settings: &DatabaseSettings) -> Result<PgPool> {
    let options = PgPoolOptions::new()
        .max_connections(settings.pool.max_connection)
        .max_lifetime(Some(Duration::from_secs(settings.pool.max_lifetime)))
        .idle_timeout(Some(Duration::from_secs(settings.pool.idle_timeout)))
        .acquire_timeout(Duration::from_secs(settings.pool.acquire_timeout))
        .acquire_timeout(Duration::from_secs(1));

    Ok(options.connect_lazy_with(with_db(settings)))
}

pub fn with_db(database_settings: &DatabaseSettings) -> PgConnectOptions {
    without_db(database_settings).database(&database_settings.database_name)
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
