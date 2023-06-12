use crate::message_bus::{Message, MessageHandler};

use async_trait::async_trait;
use derive_new::new;
use perroute_commons::types::actor::Actor;
use perroute_storage::models::channel::Channel;
use serde::Serialize;

#[derive(Debug, new, Serialize, Clone)]
pub struct Command;

impl Message for Command {}

#[derive(Debug, new)]
pub struct Handler;

#[derive(thiserror::Error, Debug)]
pub enum Error {}

#[async_trait]
impl MessageHandler for Handler {
    type Message = Command;

    type Output = Vec<Channel>;

    type Error = Error;

    async fn handle(
        &self,
        actor: Actor,
        _message: Self::Message,
    ) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
