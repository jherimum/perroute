use crate::{
    cqrs::message_bus::{Message, MessageHandler},
    errors::OmniMessageError,
    storage::database_models::connection::{Connection, ConnectionsQuery},
    types::OmniResult,
};
use async_trait::async_trait;
use sqlx::PgPool;

#[derive(Debug)]
pub struct Query;

impl Message for Query {}

#[derive(Debug)]
pub struct Handler {
    pool: PgPool,
}

#[async_trait]
impl MessageHandler for Handler {
    type Message = Query;
    type Output = Vec<Connection>;

    async fn handle(&self, message: Self::Message) -> OmniResult<Self::Output> {
        Connection::query(&self.pool, message.into())
            .await
            .map_err(OmniMessageError::from)
    }
}

impl From<Query> for ConnectionsQuery {
    fn from(value: Query) -> Self {
        todo!()
    }
}
