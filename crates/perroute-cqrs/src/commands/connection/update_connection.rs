use crate::message_bus::{Message, MessageHandler};
use async_trait::async_trait;
use perroute_storage::models::connection::Connection;
use serde_json::Value;
use sqlx::PgPool;

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
    type Error = Error;
    async fn handle(&self, _message: Self::Message) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
