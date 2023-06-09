use crate::{
    connector::Plugins,
    cqrs::message_bus::{Message, MessageHandler},
    storage::database_models::connection::Connection,
};
use anyhow::Context;
use async_trait::async_trait;
use serde_json::Value;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Debug)]
pub struct Command {
    pub code: String,
    pub plugin_id: String,
    pub properties: Value,
    pub description: String,
}

impl Message for Command {}

#[derive(Debug)]
pub struct CommandHandler {
    pool: Arc<PgPool>,
    plugins: Plugins,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),

    #[error("Plugin with id {0} does not exists")]
    PluginNotFound(String),

    #[error("A connection with code {0} already exists")]
    ConnectorCodeAlreadyExists(String),
}

#[async_trait]
impl MessageHandler for CommandHandler {
    type Message = Command;
    type Output = Connection;
    type Error = Error;

    async fn handle(&self, message: Self::Message) -> Result<Self::Output, Self::Error> {
        let plugin = self
            .plugins
            .get(&message.plugin_id)
            .ok_or(Error::PluginNotFound(message.plugin_id.to_owned()))?;

        if Connection::exists_code(self.pool.as_ref(), &message.code)
            .await
            .with_context(|| "")?
        {
            return Err(Error::ConnectorCodeAlreadyExists(message.code.to_owned()));
        }
        Connection::new(
            message.code,
            plugin,
            message.description,
            &message.properties,
        )
        .save(self.pool.as_ref())
        .await
        .with_context(|| "context")
        .map_err(Error::from)
    }
}
