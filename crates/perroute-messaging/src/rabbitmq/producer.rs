use super::{
    connection::{RabbitmqChannel, RabbitmqConnection},
    RoutingKey,
};
use lapin::options::ConfirmSelectOptions;
use serde::Serialize;
use std::fmt::Debug;
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum ProducerError {
    #[error(transparent)]
    LapinError(#[from] lapin::Error),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error("Message was not acked")]
    NotAcked,

    #[error(transparent)]
    ConnectionError(#[from] super::connection::ConnectionError),
}

#[derive(Debug, Clone)]
pub struct Producer {
    channel: RabbitmqChannel,
    exchange: String,
    confirm_select: bool,
}

impl Producer {
    pub async fn new(
        conn: RabbitmqConnection,
        exchange: &str,
        confirm_select: bool,
    ) -> Result<Producer, ProducerError> {
        Ok(Self {
            channel: conn.create_channel(),
            exchange: exchange.to_string(),
            confirm_select,
        })
    }

    pub async fn send<M: Serialize + Debug + Send>(
        &self,
        message: &M,
        routing_key: Option<RoutingKey>,
    ) -> Result<(), ProducerError> {
        let json = serde_json::to_string(&message)
            .tap_err(|e| tracing::error!("Failed to serialize message: {e}"))?;

        let channel = self.channel.get().await?;

        if self.confirm_select {
            channel
                .confirm_select(ConfirmSelectOptions::default())
                .await?;
        }

        match channel
            .basic_publish(
                &self.exchange,
                routing_key.as_ref().map(|r| r.as_ref()).unwrap_or(""),
                lapin::options::BasicPublishOptions::default(),
                json.as_bytes(),
                lapin::BasicProperties::default(),
            )
            .await
        {
            Ok(publisher_confirmation) => {
                channel.wait_for_confirms().await?;

                let confirmation = publisher_confirmation.await.map_err(ProducerError::from)?;
                if self.confirm_select && !confirmation.is_ack() {
                    return Err(ProducerError::NotAcked);
                }
                Ok(())
            }
            Err(e) => {
                tracing::error!("Failed to publish message: {e}");
                Err(e.into())
            }
        }
    }
}
