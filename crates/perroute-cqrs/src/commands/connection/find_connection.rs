use crate::message_bus::{Message, MessageHandler};
use async_trait::async_trait;
use perroute_commons::types::actor::Actor;
use perroute_storage::models::connection::Connection;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, Serialize, Clone)]
pub struct Query(pub uuid::Uuid);

impl Message for Query {}

#[derive(Debug)]
pub struct Handler {
    pool: PgPool,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {}

#[async_trait]
impl MessageHandler for Handler {
    type Message = Query;
    type Output = Option<Connection>;
    type Error = Error;

    async fn handle(
        &self,
        actor: Actor,
        _message: Self::Message,
    ) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
