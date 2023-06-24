use crate::rest::routes::channels_routes::ChannelRouter;
use crate::rest::routes::health::HealthRouter;
use crate::rest::routes::message_types_routes::MessageTypeRouter;
use crate::rest::Buses;
use anyhow::{Context, Result};
use axum::Router;
use perroute_commons::configuration::settings::Settings;
use perroute_cqrs::command_bus::bus::CommandBus;
use perroute_cqrs::command_bus::handlers::channel::create_channel::CreateChannelCommandHandler;
use perroute_cqrs::command_bus::handlers::channel::delete_channel::DeleteChannelCommandHandler;
use perroute_cqrs::command_bus::handlers::channel::update_channel::UpdateChannelCommandHandler;
use perroute_cqrs::command_bus::handlers::message_type::create_message_type::CreateMessageTypeCommandHandler;
use perroute_cqrs::query_bus::bus::QueryBus;
use perroute_cqrs::query_bus::handlers::channel::find_channel::FindChannelQueryHandler;
use perroute_cqrs::query_bus::handlers::channel::query_channels::QueryChannelsQueryHandler;
use perroute_cqrs::query_bus::handlers::message_type::find_message_type::FindMessageTypeQueryHandler;
use perroute_cqrs::query_bus::handlers::message_type::query_message_types::QueryMessageTypesHandler;
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
        let query_bus = QueryBus::builder()
            .with_pool(self.pool.clone())
            .with_handler(FindChannelQueryHandler)
            .with_handler(QueryChannelsQueryHandler)
            .with_handler(QueryMessageTypesHandler)
            .with_handler(FindMessageTypeQueryHandler)
            .build();
        let command_bus = CommandBus::builder()
            .with_pool(self.pool.clone())
            .with_handler(CreateChannelCommandHandler)
            .with_handler(DeleteChannelCommandHandler)
            .with_handler(UpdateChannelCommandHandler)
            .with_handler(CreateMessageTypeCommandHandler)
            .build();
        let buses = Buses::new(command_bus, query_bus);

        let app = Router::new().nest("/", HealthRouter::routes()).nest(
            "/api",
            Router::new()
                .merge(ChannelRouter::routes(buses.clone()))
                .merge(MessageTypeRouter::routes(buses)),
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
