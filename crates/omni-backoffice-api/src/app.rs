use std::{
    net::{AddrParseError, SocketAddr},
    str::FromStr,
};

use anyhow::Result;
use axum::Router;
use omni_cqrs::message_bus::MessageBus;
use sqlx::PgPool;
use tokio::signal;

use crate::{
    configuration::{ApplicationSettings, Settings},
    rest::routes::{channels, connections, health},
};

impl TryFrom<&ApplicationSettings> for SocketAddr {
    type Error = AddrParseError;

    fn try_from(value: &ApplicationSettings) -> Result<Self, Self::Error> {
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
            addr: (&settings.application).try_into()?,
            pool: (&settings.database).try_into()?,
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
