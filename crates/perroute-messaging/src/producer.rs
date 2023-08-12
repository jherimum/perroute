use crate::connection::RecoverableConnection;
use lapin::{options::ConfirmSelectOptions, publisher_confirm::Confirmation, Channel};
use serde::Serialize;
use std::{fmt::Debug, sync::Arc};
use tap::TapFallible;
use tokio::sync::RwLock;

#[derive(Debug, thiserror::Error)]
pub enum ProducerError {
    #[error(transparent)]
    LapinError(#[from] lapin::Error),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error("Message was not acked")]
    NotAcked,
}

#[derive(Debug, Clone)]
pub struct Producer<'c> {
    connection: &'c RecoverableConnection,
    channel: Arc<RwLock<Channel>>,
    exchange: String,
    confirm_select: bool,
}

impl<'c> Producer<'c> {
    pub async fn new(
        conn: &'c RecoverableConnection,
        exchange: &str,
        confirm_select: bool,
    ) -> Result<Producer<'c>, ProducerError> {
        Ok(Self {
            channel: Arc::new(RwLock::new(
                Self::create_channel(&conn, confirm_select)
                    .await
                    .tap_err(|e| tracing::error!("Failed to create channel: {e}"))?,
            )),
            connection: conn,
            exchange: exchange.to_owned(),
            confirm_select,
        })
    }

    async fn create_channel(
        conn: &RecoverableConnection,
        confirm_select: bool,
    ) -> Result<Channel, lapin::Error> {
        let channel = conn
            .create_channel()
            .await
            .tap_err(|e| tracing::error!("Failed to create channel: {e}"))?;
        if confirm_select {
            channel
                .confirm_select(ConfirmSelectOptions::default())
                .await
                .tap_err(|e| tracing::error!("Failed to confirm select: {e}"))?
        }
        Ok(channel)
    }

    async fn recreate_channel(&self) -> Result<(), lapin::Error> {
        let mut channel = self.channel.write().await;
        *channel = Self::create_channel(&self.connection, self.confirm_select)
            .await
            .tap_err(|e| tracing::error!("Failed to recreate channel: {e}"))?;
        Ok(())
    }

    async fn send_message<M: Serialize + Debug>(
        &self,
        channel: &Channel,
        message: &M,
        routing_key: Option<&str>,
    ) -> Result<Confirmation, ProducerError> {
        let json = serde_json::to_string(&message)
            .tap_err(|e| tracing::error!("Failed to serialize message: {e}"))?;
        match channel
            .basic_publish(
                &self.exchange,
                routing_key.unwrap_or(""),
                lapin::options::BasicPublishOptions::default(),
                json.as_bytes(),
                lapin::BasicProperties::default(),
            )
            .await
        {
            Ok(r) => {
                let x = r.await.map_err(ProducerError::from)?;
                if self.confirm_select && !x.is_ack() {
                    return Err(ProducerError::NotAcked);
                }
                Ok(x)
            }
            Err(e) => {
                tracing::error!("Failed to publish message: {e}");
                Err(e.into())
            }
        }
    }

    pub async fn send<M: Serialize + Debug>(
        &self,
        message: &M,
        routing_key: Option<&str>,
    ) -> Result<(), ProducerError> {
        {
            let channel = self.channel.read().await;
            if channel.status().connected() {
                self.send_message(&channel, message, routing_key)
                    .await
                    .tap_err(|e| tracing::error!("Failed to send message: {e}"))?;
                return Ok(());
            }
        }
        tracing::warn!("Channel is not connected, recreating it...");
        self.recreate_channel().await?;
        let channel = &self.channel.read().await;
        self.send_message(&channel, message, routing_key)
            .await
            .tap_err(|e| tracing::error!("Failed to send message: {e}"))?;
        Ok(())
    }
}
