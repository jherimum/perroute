use async_trait::async_trait;

use crate::{
    connector::{ConnectoPlugin, Plugins},
    cqrs::message_bus::{Message, MessageHandler},
    types::OmniResult,
};

#[derive(Debug)]
pub struct QueryPluginsMessage;

impl Message for QueryPluginsMessage {}

#[derive(Debug)]
pub struct QueryPluginsHandler {
    pub plugins: Plugins,
}

#[async_trait]
impl MessageHandler for QueryPluginsHandler {
    type Message = QueryPluginsMessage;

    type Output = Vec<&'static dyn ConnectoPlugin>;

    async fn handle(&self, _: QueryPluginsMessage) -> OmniResult<Vec<&'static dyn ConnectoPlugin>> {
        Ok(self.plugins.all())
    }
}
