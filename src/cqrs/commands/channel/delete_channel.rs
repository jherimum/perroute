use crate::{
    cqrs::message_bus::{Message, MessageHandler},
    storage::database_models::channel::Channel,
    types::ArcPool,
};
use anyhow::Context;
use async_trait::async_trait;

#[derive(Debug)]
pub struct Command {
    id: uuid::Uuid,
}

impl Command {
    pub fn new(id: uuid::Uuid) -> Self {
        Self { id }
    }
}

impl Message for Command {}

#[derive(Debug)]
pub struct Handler {
    pool: ArcPool,
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

    async fn handle(&self, message: Self::Message) -> Result<Self::Output, Self::Error> {
        match Channel::find(self.pool.as_ref(), &message.id)
            .await
            .with_context(|| format!("Error while retrieving channel {}", message.id))?
        {
            Some(c) => Ok(c
                .delete(self.pool.as_ref())
                .await
                .with_context(|| format!("Error while deleting channel {}", message.id))?),

            None => Ok(false),
        }
    }
}
