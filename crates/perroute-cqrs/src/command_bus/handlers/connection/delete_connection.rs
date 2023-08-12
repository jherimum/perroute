use crate::query_bus::{Message, MessageHandler};
use async_trait::async_trait;
use perroute_commons::types::actor::Actor;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, Serialize, Clone)]
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
    type Error = Error;

    async fn handle(
        &self,
        actor: Actor,
        _message: Self::Message,
    ) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
