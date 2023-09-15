use async_recursion::async_recursion;
use deadpool_lapin::{Config, Pool, PoolConfig, Runtime};
use lapin::Channel;
use perroute_commons::configuration::settings::Settings;
use std::{fmt::Debug, sync::Arc, time::Duration};
use tokio::{sync::RwLock, time::error::Elapsed};

#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    #[error(transparent)]
    LapinError(#[from] lapin::Error),

    #[error(transparent)]
    ConnectionTimeout(#[from] Elapsed),

    #[error(transparent)]
    PoolError(#[from] deadpool_lapin::CreatePoolError),
}

#[derive(Debug, Clone)]
pub struct RabbitmqConnection {
    pool: Pool,
}

impl RabbitmqConnection {
    pub async fn connect(uri: &str) -> Result<RabbitmqConnection, ConnectionError> {
        Self::from_config(deadpool_lapin::Config {
            url: Some(uri.to_string()),
            ..deadpool_lapin::Config::default()
        })
        .await
    }

    pub async fn connect_from_settings(
        settings: &Settings,
    ) -> Result<RabbitmqConnection, ConnectionError> {
        let rabbitmq_settings = settings.rabbitmq.as_ref().unwrap();
        let pool_config = if let Some(pool_settings) = rabbitmq_settings.pool.as_ref() {
            let mut pool_config = PoolConfig::default();
            if let Some(max_connection) = pool_settings.max_connection {
                pool_config.max_size = max_connection;
            }
            pool_config
        } else {
            PoolConfig::default()
        };

        let config = Config {
            url: Some(rabbitmq_settings.uri.clone()),
            pool: Some(pool_config),
            ..Config::default()
        };
        Self::from_config(config).await
    }

    async fn from_config(config: Config) -> Result<RabbitmqConnection, ConnectionError> {
        Ok(Self {
            pool: config.create_pool(Some(Runtime::Tokio1))?,
        })
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
            let mut channel = self.channel.write().await;
            let new_channel = self
                .connection
                .pool
                .get()
                .await
                .unwrap()
                .create_channel()
                .await?;
            channel.replace(new_channel);
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
