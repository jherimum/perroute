use async_recursion::async_recursion;
use lapin::{Channel, Connection, ConnectionProperties};
use perroute_commons::configuration::settings::Settings;
use std::{fmt::Debug, sync::Arc, time::Duration};
use tap::TapFallible;
use tokio::{
    sync::RwLock,
    time::{error::Elapsed, Instant},
};

#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    #[error(transparent)]
    LapinError(#[from] lapin::Error),

    #[error(transparent)]
    ConnectionTimeout(#[from] Elapsed),
}

impl From<&Settings> for Config {
    fn from(value: &Settings) -> Self {
        Self {
            uri: value.rabbitmq.as_ref().unwrap().uri.clone(),
            time_out: Duration::from_secs(20),
            retry_delay: Duration::from_secs(1),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub uri: String,
    pub time_out: Duration,
    pub retry_delay: Duration,
}

#[derive(Debug, Clone)]
pub struct RabbitmqConnection {
    config: Config,
    connection: Arc<RwLock<Option<Connection>>>,
}

impl RabbitmqConnection {
    pub async fn connect(config: Config) -> Result<RabbitmqConnection, ConnectionError> {
        Ok(Self {
            config: config.clone(),
            connection: Arc::new(RwLock::new(Some(Self::connection(&config).await?))),
        })
    }

    async fn inner_connect(config: &Config) -> Result<Connection, lapin::Error> {
        Connection::connect(&config.uri, ConnectionProperties::default())
            .await
            .tap_err(|e| tracing::error!("Failed to connect to RabbitMQ: {e}"))
    }

    async fn try_connect(config: &Config) -> Connection {
        let mut res = Self::inner_connect(config).await;
        while res.is_err() {
            tokio::time::sleep(config.retry_delay).await;
            tracing::info!("Retrying to connect to RabbitMQ...");
            res = Self::inner_connect(config).await;
        }
        res.unwrap()
    }

    async fn connection(config: &Config) -> Result<Connection, ConnectionError> {
        tracing::info!("Connecting to RabbitMQ...");
        tokio::time::timeout_at(Instant::now() + config.time_out, async move {
            let conn = Self::try_connect(config).await;
            tracing::info!("Connected to RabbitMQ.");
            conn
        })
        .await
        .map_err(ConnectionError::from)
    }

    #[async_recursion]
    async fn inner_create_channel(&self) -> Result<Channel, ConnectionError> {
        {
            let conn = self.connection.read().await;
            if let Some(c) = conn.as_ref() {
                if c.status().connected() {
                    return c.create_channel().await.map_err(ConnectionError::from);
                }
            }
        }
        {
            let mut conn = self.connection.write().await;
            *conn = Some(Self::connection(&self.config).await?);
        }

        self.inner_create_channel().await
    }

    pub fn create_channel(&self) -> RabbitmqChannel {
        RabbitmqChannel::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct RabbitmqChannel {
    channel: Arc<RwLock<Option<Channel>>>,
    connection: RabbitmqConnection,
}

impl RabbitmqChannel {
    #[async_recursion]
    pub async fn get(&self) -> Result<Channel, ConnectionError> {
        {
            let channel = self.channel.read().await;
            if let Some(c) = channel.as_ref() {
                if c.status().connected() {
                    return Ok(c.clone());
                } else {
                    tracing::warn!("Channel is not connected, reconnecting...")
                }
            }
        }
        {
            self.channel
                .write()
                .await
                .replace(self.connection.inner_create_channel().await?);
        }

        self.get().await
    }

    pub fn new(connection: RabbitmqConnection) -> Self {
        Self {
            channel: Arc::new(RwLock::new(None)),
            connection,
        }
    }
}
