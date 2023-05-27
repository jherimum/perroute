use crate::{
    cqrs::message_bus::{Message, MessageHandler},
    types::OmniResult,
};
use async_trait::async_trait;
use sqlx::PgPool;

#[derive(Debug)]
pub struct Command {
    pub id: uuid::Uuid,
}

impl Message for Command {}

#[derive(Debug)]
pub struct Handler {
    pub pool: PgPool,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {}

#[async_trait]
impl MessageHandler for Handler {
    type Message = Command;

    type Output = ();

    async fn handle(&self, message: Self::Message) -> OmniResult<Self::Output> {
        todo!()
    }
}
