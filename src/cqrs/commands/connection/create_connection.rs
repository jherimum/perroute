use crate::{
    connector::Plugins,
    cqrs::message_bus::{Message, MessageHandler},
    database_models::connection::Connection,
    errors::OmniMessageError,
    types::OmniResult,
};
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
    Database(#[from] sqlx::Error),

    #[error("Plugin with id {0} does not exists")]
    PluginNotFound(String),

    #[error("A connection with code {0} already exists")]
    ConnectorCodeAlreadyExists(String),
}

impl From<Error> for OmniMessageError {
    fn from(value: Error) -> Self {
        todo!()
    }
}

#[async_trait]
impl MessageHandler for CommandHandler {
    type Message = Command;

    type Output = Connection;

    async fn handle(&self, message: Self::Message) -> OmniResult<Self::Output> {
        let plugin = self
            .plugins
            .get(&message.plugin_id)
            .ok_or(Error::PluginNotFound(message.plugin_id.to_owned()))?;

        if Connection::exists_by_account_id_and_code(self.pool.as_ref(), &message.code).await? {
            return Err(Error::ConnectorCodeAlreadyExists(message.code.to_owned()).into());
        }
        Connection::new(
            &message.code,
            plugin,
            &message.description,
            &message.properties,
        )
        .save(self.pool.as_ref())
        .await
        .map_err(OmniMessageError::from)
    }
}
