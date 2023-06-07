use crate::rest::routes::{channels, connections, health};
use anyhow::{Context, Result};
use axum::Router;
use omni_commons::configuration::settings::Settings;
use omni_cqrs::message_bus::MessageBus;
use omni_storage::connection_manager::{MigrantionMode, OmniMessageConnectionManager};
use sqlx::PgPool;
use std::net::SocketAddr;
use tokio::signal;

pub struct App {
    pool: PgPool,
    addr: SocketAddr,
}

impl App {
    pub async fn from_settings(settings: &Settings) -> Result<Self> {
        Ok(Self {
            addr: SocketAddr::try_from(&settings.server).context("context")?,
            pool: OmniMessageConnectionManager::new_pool(
                &settings.database,
                MigrantionMode::Skip,
                None,
                vec![],
            )
            .await?,
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
