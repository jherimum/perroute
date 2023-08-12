use crate::query_bus::{Message, MessageHandler};
use async_trait::async_trait;
use perroute_commons::types::actor::Actor;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
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

    async fn handle(
        &self,
        actor: Actor,
        _message: Self::Message,
    ) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
