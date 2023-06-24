use crate::rest::routes::channel_router::ChannelRouter;
use crate::rest::routes::health_router::HealthRouter;
use crate::rest::routes::message_type_router::MessageTypeRouter;
use crate::rest::routes::message_type_version_router::MessageTypeVersionRouter;
use crate::rest::Buses;
use anyhow::{Context, Result};
use axum::Router;
use perroute_commons::configuration::settings::Settings;
use perroute_cqrs::command_bus::bus::CommandBus;
use perroute_cqrs::query_bus::bus::QueryBus;
use perroute_storage::connection_manager::ConnectionManager;
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
            pool: ConnectionManager::build_pool(&settings.database).await?,
        })
    }

    pub fn new(pool: PgPool, addr: SocketAddr) -> Self {
        Self { pool, addr }
    }

    pub async fn init(self) -> Result<()> {
        let buses = Buses::new(
            CommandBus::complete(self.pool.clone()),
            QueryBus::complete(self.pool.clone()),
        );

        let app = Router::new().nest("/", HealthRouter::routes()).nest(
            "/api",
            Router::new()
                .merge(ChannelRouter::routes(buses.clone()))
                .merge(MessageTypeRouter::routes(buses.clone()))
                .merge(MessageTypeVersionRouter::routes(buses)),
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
