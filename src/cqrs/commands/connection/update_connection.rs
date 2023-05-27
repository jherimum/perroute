use async_trait::async_trait;
use serde_json::Value;
use sqlx::PgPool;

use crate::{
    cqrs::message_bus::{Message, MessageHandler},
    storage::database_models::connection::Connection,
    types::OmniResult,
};

#[derive(Debug)]
pub struct Command {
    pub id: uuid::Uuid,
    pub description: Option<String>,
    pub properties: Option<Value>,
}

impl Message for Command {}

#[derive(Debug)]
pub struct Handler {
    pool: PgPool,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {}

#[async_trait]
impl MessageHandler for Handler {
    type Message = Command;

    type Output = Connection;

    async fn handle(&self, message: Self::Message) -> OmniResult<Self::Output> {
        todo!()
    }
}
