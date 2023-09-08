use events::Event;
use std::fmt::Debug;

pub mod events;
pub mod rabbitmq;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct EventPublisherError(#[from] anyhow::Error);

#[async_trait::async_trait]
pub trait EventPublisher: Debug + Clone {
    async fn publish(&self, event: &Event) -> Result<(), EventPublisherError>;
}
