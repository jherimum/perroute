use crate::{
    connector::{ConnectorPlugin, Plugins},
    cqrs::message_bus::{Message, MessageHandler},
};
use async_trait::async_trait;

#[derive(Debug)]
pub struct QueryPluginsMessage;

impl Message for QueryPluginsMessage {}

#[derive(Debug)]
pub struct QueryPluginsHandler {
    pub plugins: Plugins,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {}

#[async_trait]
impl MessageHandler for QueryPluginsHandler {
    type Message = QueryPluginsMessage;
    type Output = Vec<&'static dyn ConnectorPlugin>;
    type Error = Error;

    async fn handle(&self, _: QueryPluginsMessage) -> Result<Self::Output, Self::Error> {
        Ok(self.plugins.all())
    }
}
