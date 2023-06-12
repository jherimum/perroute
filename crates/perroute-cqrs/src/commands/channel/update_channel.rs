use crate::{
    actor::Actor,
    message_bus::{Message, MessageHandler},
};
use async_trait::async_trait;
use derive_new::new;
use perroute_commons::types::id::Id;
use perroute_storage::models::channel::Channel;
use sqlx::PgPool;
use tap::TapFallible;

#[derive(Debug, new)]
pub struct Command {
    id: Id,
    name: String,
}

impl Message for Command {}

#[derive(Debug, new)]
pub struct Handler {
    pool: PgPool,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),

    #[error("Channel with id {0} nor found")]
    ChannelNotFound(Id),
}

#[async_trait]
impl MessageHandler for Handler {
    type Message = Command;
    type Output = Channel;
    type Error = Error;

    #[tracing::instrument(skip(self))]
    async fn handle(
        &self,
        actor: Actor,
        message: Self::Message,
    ) -> Result<Self::Output, Self::Error> {
        let mut channel = retrieve_channel(&self.pool, &message.id).await?;

        channel.with_name(message.name);

        Ok(channel
            .update(&self.pool)
            .await
            .tap_err(|e| {
                tracing::error!("Error while updating channel {}. Error: {}", message.id, e)
            })
            .map_err(anyhow::Error::from)?)
    }
}

async fn retrieve_channel(pool: &PgPool, id: &Id) -> Result<Channel, Error> {
    Channel::find_by_id(pool, id)
        .await
        .tap_err(|e| tracing::error!("Error while retrieving channel {}. Error: {}", id, e))
        .map_err(anyhow::Error::new)?
        .ok_or_else(|| Error::ChannelNotFound(*id))
}
