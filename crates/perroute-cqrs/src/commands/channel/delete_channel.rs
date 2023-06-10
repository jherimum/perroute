use crate::{
    actor::Actor,
    message_bus::{Message, MessageHandler},
};
use anyhow::Context;
use async_trait::async_trait;
use perroute_commons::types::id::Id;
use perroute_storage::models::channel::Channel;
use sqlx::PgPool;

#[derive(Debug)]
pub struct Command {
    id: Id,
}

impl Command {
    pub fn new(id: Id) -> Self {
        Self { id }
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
}

#[async_trait]
impl MessageHandler for Handler {
    type Message = Command;

    type Output = bool;

    type Error = Error;

    async fn handle(
        &self,
        actor: Actor,
        message: Self::Message,
    ) -> Result<Self::Output, Self::Error> {
        match Channel::find_by_id(&self.pool, &message.id)
            .await
            .with_context(|| format!("Error while retrieving channel {}", message.id))?
        {
            Some(c) => Ok(c
                .delete(&self.pool)
                .await
                .with_context(|| format!("Error while deleting channel {}", message.id))?),

            None => Ok(false),
        }
    }
}
