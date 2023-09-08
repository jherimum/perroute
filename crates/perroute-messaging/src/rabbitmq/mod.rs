use self::{connection::RecoverableConnection, producer::Producer};
use crate::{events::Event, EventPublisher, EventPublisherError};
use anyhow::anyhow;

pub mod connection;
pub mod consumer;
pub mod producer;

pub struct RoutingKey(String);

impl AsRef<str> for RoutingKey {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&Event> for RoutingKey {
    fn from(value: &Event) -> Self {
        Self(value.ty().to_string())
    }
}

#[derive(Clone, Debug)]
pub struct RabbitmqEventPublisher {
    producer: producer::Producer,
}

impl RabbitmqEventPublisher {
    pub async fn new(
        conn: RecoverableConnection,
        exchange: &str,
        confirm_select: bool,
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            producer: Producer::new(conn, exchange, confirm_select).await.unwrap(),
        })
    }
}

#[async_trait::async_trait]
impl EventPublisher for RabbitmqEventPublisher {
    async fn publish(&self, event: &Event) -> Result<(), EventPublisherError> {
        let routing_key = RoutingKey::from(event);
        self.producer
            .send(event, Some(routing_key))
            .await
            .map_err(|e| EventPublisherError::from(anyhow!(e)))
    }
}
