use crate::{
    cqrs::message_bus::{Message, MessageHandler},
    storage::database_models::connection::{Connection, ConnectionsQuery},
};
use anyhow::Context;
use async_trait::async_trait;
use sqlx::PgPool;

#[derive(Debug)]
pub struct Query;

impl Message for Query {}

#[derive(Debug)]
pub struct Handler {
    pool: PgPool,
}
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[async_trait]
impl MessageHandler for Handler {
    type Message = Query;
    type Output = Vec<Connection>;
    type Error = Error;

    async fn handle(&self, message: Self::Message) -> Result<Self::Output, Self::Error> {
        Connection::query(&self.pool, message.into())
            .await
            .with_context(|| "")
            .map_err(Error::from)
    }
}

impl From<Query> for ConnectionsQuery {
    fn from(value: Query) -> Self {
        todo!()
    }
}
