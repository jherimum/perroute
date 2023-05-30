use crate::{
    cqrs::message_bus::{Message, MessageHandler},
    storage::database_models::channel::Channel,
};
use async_trait::async_trait;

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
