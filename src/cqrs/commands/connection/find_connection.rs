use crate::{
    cqrs::message_bus::{Message, MessageHandler},
    storage::database_models::connection::Connection,
};
use async_trait::async_trait;
use sqlx::PgPool;
use std::todo;

#[derive(Debug)]
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

    async fn handle(&self, message: Self::Message) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
