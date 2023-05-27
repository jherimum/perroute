use std::todo;

use async_trait::async_trait;

use crate::{
    cqrs::message_bus::{Message, MessageHandler},
    types::OmniResult,
};

#[derive(Debug)]
pub struct CreateTokenCommand {}

impl Message for CreateTokenCommand {}

#[derive(Debug)]
pub struct CreateTokenHandler {}

#[derive(Debug, thiserror::Error)]
pub enum CreateTokenError {}

#[async_trait]
impl MessageHandler for CreateTokenHandler {
    type Message = CreateTokenCommand;
    type Output = String;
    type Error = CreateTokenError;

    async fn handle(&self, message: Self::Message) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
