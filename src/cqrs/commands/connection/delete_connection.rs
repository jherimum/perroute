use async_trait::async_trait;
use sqlx::PgPool;

use crate::{
    cqrs::message_bus::{Message, MessageHandler},
    database_models::account::Account,
};

#[derive(Debug)]
pub struct Command {
    pub id: uuid::Uuid,
    pub account: Account,
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

    async fn handle(&self, message: Self::Message) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
