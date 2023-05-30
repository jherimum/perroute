use crate::{
    cqrs::message_bus::{Message, MessageHandler},
    storage::database_models::channel::Channel,
};
use anyhow::Context;
use async_trait::async_trait;
use sqlx::PgPool;

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

    type Output = Option<Channel>;

    type Error = Error;

    async fn handle(&self, message: Self::Message) -> Result<Self::Output, Self::Error> {
        Ok(Channel::find(&self.pool, &message.id)
            .await
            .with_context(|| format!("Error while retrieve channel {}", message.id))?)
    }
}
