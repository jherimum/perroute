use crate::{
    cqrs::message_bus::{Message, MessageHandler},
    storage::database_models::channel::Channel,
};
use async_trait::async_trait;
use sqlx::PgPool;
use tap::TapFallible;

#[derive(Debug)]
pub struct Command {
    id: uuid::Uuid,
    description: Option<String>,
}

impl Command {
    pub fn new(id: uuid::Uuid, desc: Option<impl Into<String>>) -> Self {
        Self {
            id,
            description: desc.map(Into::into),
        }
    }
}

impl Message for Command {}

#[derive(Debug)]
pub struct Handler {
    pool: PgPool,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),

    #[error("Channel with id {0} nor found")]
    ChannelNotFound(uuid::Uuid),
}

#[async_trait]
impl MessageHandler for Handler {
    type Message = Command;
    type Output = Channel;
    type Error = Error;

    async fn handle(&self, message: Self::Message) -> Result<Self::Output, Self::Error> {
        Ok(Channel::find(&self.pool, &message.id)
            .await
            .tap_err(|e| {
                tracing::error!(
                    "Error while retrieving channel {}. Error: {}",
                    message.id,
                    e
                )
            })
            .map_err(anyhow::Error::new)?
            .ok_or_else(|| Error::ChannelNotFound(message.id))?
            .update(&self.pool, message.description)
            .await
            .tap_err(|e| {
                tracing::error!("Error while updating channel {}. Error: {}", message.id, e)
            })
            .map_err(anyhow::Error::new)?)
    }
}
