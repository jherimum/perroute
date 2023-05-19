use std::todo;

use async_trait::async_trait;
use sqlx::PgPool;

use crate::{
    cqrs::message_bus::{Message, MessageHandler},
    database_models::{account::Account, connection::Connection},
    types::OmniResult,
};

#[derive(Debug)]
pub struct Query {
    pub id: uuid::Uuid,
    pub account: Account,
}

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

    async fn handle(&self, message: Self::Message) -> OmniResult<Self::Output> {
        todo!()
    }
}
