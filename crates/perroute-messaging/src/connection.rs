use lapin::{Channel, Connection, ConnectionProperties};
use std::{fmt::Debug, sync::Arc, time::Duration};
use tap::TapFallible;
use tokio::{sync::RwLock, time::Instant};

#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    #[error(transparent)]
    LapinError(#[from] lapin::Error),
}

#[derive(Debug, Clone)]
pub struct RecoverableConnection {
    config: Config,
    connection: Arc<RwLock<Connection>>,
}

impl RecoverableConnection {
    pub async fn connect(config: Config) -> Self {
        Self {
            connection: Arc::new(RwLock::new(Self::connection(&config.uri).await)),
            config: config,
        }
    }

    async fn connection(uri: &str) -> Connection {
        tokio::time::timeout_at(Instant::now() + Duration::from_secs(20), async move {
            let mut res = Connection::connect(&uri, ConnectionProperties::default())
                .await
                .tap_err(|e| tracing::error!("Failed to connect to RabbitMQ: {e}"));
            while res.is_err() {
                tokio::time::sleep(Duration::from_secs(2)).await;
                tracing::info!("Retrying to connect to RabbitMQ...");
                res = Connection::connect(&uri, ConnectionProperties::default())
                    .await
                    .tap_err(|e| tracing::error!("Failed to connect to RabbitMQ: {e}"));
            }

            res.tap_ok(|c| tracing::info!("Connected to RabbitMQ: {:?}", c))
                .unwrap()
        })
        .await
        .expect("Failed to connect to RabbitMQ")
    }

    async fn refresh(&self) {
        tracing::info!("Refreshing connection to RabbitMQ...");
        let mut conn = self.connection.write().await;
        *conn = Self::connection(&self.config.uri).await;
    }

    pub(crate) async fn create_channel(&self) -> Result<Channel, lapin::Error> {
        {
            let conn = self.connection.read().await;
            if conn.status().connected() {
                return Ok(conn.create_channel().await?);
            }
        }

        self.refresh().await;
        self.connection.read().await.create_channel().await
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub uri: String,
}
