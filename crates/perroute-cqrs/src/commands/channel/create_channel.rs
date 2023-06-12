use anyhow::Context;
use async_trait::async_trait;
use derive_new::new;
use perroute_commons::types::{actor::Actor, code::Code};
use perroute_storage::models::{channel::Channel, command_log::CommandLog};
use serde::Serialize;
use sqlx::PgPool;

use crate::message_bus::{Message, MessageHandler};

#[derive(Debug, new, Serialize, Clone)]
pub struct Command {
    code: Code,
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

    #[error("A channel with code {0} already exists")]
    CodeAlreadyExists(Code),
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
        if Channel::exists_by_code(&self.pool, &message.code)
            .await
            .with_context(|| {
                format!(
                    "Error while checking if channel with code {} exists",
                    message.code,
                )
            })?
        {
            return Err(Error::CodeAlreadyExists(message.code));
        }

        let mut tx = self.pool.begin().await.unwrap();

        let channel = Channel::new(&message.code, &message.name)
            .save(&mut tx)
            .await
            .unwrap();

        CommandLog::new("".to_owned(), actor, &message)
            .save(&mut tx)
            .await
            .unwrap();

        Ok(channel)
    }
}
