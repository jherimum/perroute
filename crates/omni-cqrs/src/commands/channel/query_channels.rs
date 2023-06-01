use crate::message_bus::{Message, MessageHandler};
use anyhow::Context;
use async_trait::async_trait;
use omni_storage::models::channel::Channel;
use sqlx::PgPool;

#[derive(Debug)]
pub struct Command;

impl Message for Command {}

#[derive(Debug)]
pub struct Handler;

#[derive(thiserror::Error, Debug)]
pub enum Error {}

#[async_trait]
impl MessageHandler for Handler {
    type Message = Command;

    type Output = Vec<Channel>;

    type Error = Error;

    async fn handle(&self, message: Self::Message) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
