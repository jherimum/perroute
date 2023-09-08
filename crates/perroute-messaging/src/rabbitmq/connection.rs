use lapin::{
    options::{QueueBindOptions, QueueDeclareOptions},
    types::FieldTable,
    Channel, Connection, ConnectionProperties, Queue,
};
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
pub struct RecoverableConnection {
    config: Config,
    connection: Arc<RwLock<Connection>>,
}

impl RecoverableConnection {
    pub async fn connect(config: Config) -> Result<Self, ConnectionError> {
        Ok(Self {
            connection: Arc::new(RwLock::new(Self::connection(&config).await?)),
            config,
        })
    }

    async fn connection(config: &Config) -> Result<Connection, ConnectionError> {
        tokio::time::timeout_at(Instant::now() + config.time_out, async move {
            let mut res = Connection::connect(&config.uri, ConnectionProperties::default())
                .await
                .tap_err(|e| tracing::error!("Failed to connect to RabbitMQ: {e}"));
            while res.is_err() {
                tokio::time::sleep(config.retry_delay).await;
                tracing::info!("Retrying to connect to RabbitMQ...");
                res = Connection::connect(&config.uri, ConnectionProperties::default())
                    .await
                    .tap_err(|e| tracing::error!("Failed to connect to RabbitMQ: {e}"));
            }

            res.tap_ok(|c| tracing::info!("Connected to RabbitMQ: {:?}", c))
                .unwrap()
        })
        .await
        .map_err(ConnectionError::from)
    }

    async fn refresh(&self) -> Result<(), ConnectionError> {
        tracing::info!("Refreshing connection to RabbitMQ...");
        let mut conn = self.connection.write().await;
        *conn = Self::connection(&self.config).await?;
        Ok(())
    }

    async fn create_channel(&self) -> Result<Channel, ConnectionError> {
        {
            let conn = self.connection.read().await;
            if conn.status().connected() {
                return Ok(conn.create_channel().await?);
            }
        }

        self.refresh().await?;
        self.connection
            .read()
            .await
            .create_channel()
            .await
            .map_err(ConnectionError::from)
    }

    pub async fn create_recoverable_channel(&self) -> RecoverableChannel {
        RecoverableChannel::new(self.clone()).await
    }
}

#[derive(Debug, Clone)]
pub struct RecoverableChannel {
    channel: Arc<RwLock<Option<Channel>>>,
    connection: RecoverableConnection,
}

impl RecoverableChannel {
    pub async fn new(connection: RecoverableConnection) -> Self {
        Self {
            channel: Arc::new(RwLock::new(None)),
            connection,
        }
    }

    async fn refresh(&self) {
        tracing::info!("Refreshing channel to RabbitMQ...");
        let mut channel = self.channel.write().await;
        *channel = Some(self.connection.create_channel().await.unwrap());
    }

    pub async fn get(&self) -> Channel {
        tracing::info!("Getting channel to RabbitMQ........................");

        {
            let channel = self.channel.read().await;

            if channel.is_some() && channel.as_ref().unwrap().status().connected() {
                return channel.clone().unwrap();
            }
        }

        self.refresh().await;
        self.channel.read().await.as_ref().unwrap().clone()
    }

    pub async fn queue_declare(
        &self,
        queue: &str,
        options: QueueDeclareOptions,
        arguments: FieldTable,
    ) -> Result<Queue, lapin::Error> {
        self.get()
            .await
            .queue_declare(queue, options, arguments)
            .await
    }

    pub async fn queue_bind(
        &self,
        queue: &str,
        exchange: &str,
        routing_key: &str,
        options: QueueBindOptions,
        arguments: FieldTable,
    ) -> Result<(), lapin::Error> {
        self.get()
            .await
            .queue_bind(queue, exchange, routing_key, options, arguments)
            .await
    }
}
