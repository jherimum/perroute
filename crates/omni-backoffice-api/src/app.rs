use crate::rest::routes::{channels, connections, health};
use anyhow::Result;
use axum::Router;
use omni_commons::configuration::DatabaseSettings;
use omni_cqrs::message_bus::MessageBus;
use omni_storage::connection::build_pool;
use serde::Deserialize;
use serde_aux::prelude::deserialize_number_from_string;
use sqlx::PgPool;
use std::{
    net::{AddrParseError, SocketAddr},
    str::FromStr,
};
use tokio::signal;

#[derive(Deserialize, Clone, Debug)]
pub struct Settings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database: DatabaseSettings,
}

impl TryFrom<&Settings> for SocketAddr {
    type Error = AddrParseError;

    fn try_from(value: &Settings) -> Result<Self, Self::Error> {
        SocketAddr::from_str(&format!("{}:{}", value.host, value.port))
    }
}

pub struct App {
    pool: PgPool,
    addr: SocketAddr,
}

impl App {
    pub fn from_settings(settings: &Settings) -> Result<Self> {
        Ok(Self {
            addr: settings.try_into()?,
            pool: build_pool(&settings.database)?,
        })
    }

    pub fn new(pool: PgPool, addr: SocketAddr) -> Self {
        Self { pool, addr }
    }

    pub async fn init(self) -> Result<()> {
        let message_bus = MessageBus::builder().build();

        let app = Router::new().nest("/healh", health::routes()).nest(
            "/api",
            Router::new()
                .merge(
                    Router::new().nest("/v1/connections", connections::routes(message_bus.clone())),
                )
                .merge(Router::new().nest("/v1/channels", channels::routes(message_bus.clone()))),
        );

        tracing::info!("listening on {}", &self.addr);

        axum::Server::bind(&self.addr)
            .serve(app.into_make_service())
            .with_graceful_shutdown(shutdown_signal())
            .await
            .unwrap();
        Ok(())
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
