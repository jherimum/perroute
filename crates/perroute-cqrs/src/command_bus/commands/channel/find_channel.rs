use crate::message_bus::{Message, MessageHandler};
use anyhow::Context;
use async_trait::async_trait;
use derive_new::new;
use perroute_commons::types::{actor::Actor, id::Id};
use perroute_storage::models::channel::Channel;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, new, Serialize, Clone)]
pub struct Command {
    id: Id,
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
}

#[async_trait]
impl MessageHandler for Handler {
    type Message = Command;

    type Output = Option<Channel>;

    type Error = Error;

    async fn handle(
        &self,
        actor: Actor,
        message: Self::Message,
    ) -> Result<Self::Output, Self::Error> {
        Ok(Channel::find_by_id(&self.pool, &message.id)
            .await
            .with_context(|| format!("Error while retrieve channel {}", message.id))?)
    }
}
