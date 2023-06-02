use crate::message_bus::{Message, MessageHandler};
use async_trait::async_trait;
use omni_storage::models::channel::Channel;
use sqlx::PgPool;
use tap::TapFallible;

#[derive(Debug)]
pub struct Command {
    id: uuid::Uuid,
    name: String,
}

impl Command {
    pub fn new(id: uuid::Uuid, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
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
        let mut channel = retrieve_channel(&self.pool, &message.id).await?;

        channel.name = message.name;

        Ok(channel
            .update(&self.pool)
            .await
            .tap_err(|e| {
                tracing::error!("Error while updating channel {}. Error: {}", message.id, e)
            })
            .map_err(anyhow::Error::from)?)
    }
}

async fn retrieve_channel(pool: &PgPool, id: &uuid::Uuid) -> Result<Channel, Error> {
    Channel::find_by_id(pool, id)
        .await
        .tap_err(|e| tracing::error!("Error while retrieving channel {}. Error: {}", id, e))
        .map_err(anyhow::Error::new)?
        .ok_or_else(|| Error::ChannelNotFound(*id))
}
