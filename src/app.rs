use std::net::SocketAddr;

use anyhow::Result;
use axum::Router;
use sqlx::PgPool;

use crate::{
    configuration::Settings,
    cqrs::message_bus::MessageBus,
    rest::routes::{channels, connections, health},
};

pub struct App {
    //pool: PgPool,
    addr: SocketAddr,
}

impl App {
    pub fn from_settings(settings: &Settings) -> Result<Self> {
        Ok(Self {
            addr: settings.try_into()?,
            //      pool: settings.try_into()?,
        })
    }

    pub fn new(
        //pool: PgPool,
        addr: SocketAddr,
    ) -> Self {
        Self {
            //pool,
            addr,
        }
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
            .await
            .unwrap();
        Ok(())
    }
}
