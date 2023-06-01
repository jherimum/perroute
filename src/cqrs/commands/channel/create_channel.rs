use crate::{
    cqrs::message_bus::{Message, MessageHandler},
    storage::database_models::channel::Channel,
};
use anyhow::Context;
use async_trait::async_trait;
use sqlx::PgPool;

#[derive(Debug)]
pub struct Command {
    code: String,
    name: String,
}

impl Command {
    pub fn new(code: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            code: code.into(),
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

    #[error("A channel with code {0} already exists")]
    CodeAlreadyExists(String),
}

#[async_trait]
impl MessageHandler for Handler {
    type Message = Command;

    type Output = Channel;

    type Error = Error;

    async fn handle(&self, message: Self::Message) -> Result<Self::Output, Self::Error> {
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

        Ok(Channel::new(message.code, message.name)
            .save(&self.pool)
            .await
            .unwrap())
    }
}
