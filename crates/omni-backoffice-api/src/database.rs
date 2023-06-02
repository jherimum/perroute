use crate::app::Settings;
use omni_commons::configuration::DatabaseSettings;
use secrecy::ExposeSecret;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    PgPool,
};
use std::time::Duration;

impl From<&Settings> for PgPool {
    fn from(value: &Settings) -> Self {
        let db_settings = &value.database;
        let options = PgPoolOptions::new()
            .max_connections(db_settings.pool.max_connection)
            .max_lifetime(Some(Duration::from_secs(db_settings.pool.max_lifetime)))
            .idle_timeout(Some(Duration::from_secs(db_settings.pool.idle_timeout)))
            .acquire_timeout(Duration::from_secs(db_settings.pool.acquire_timeout))
            .acquire_timeout(Duration::from_secs(1));

        options.connect_lazy_with(with_db(db_settings))
    }
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
