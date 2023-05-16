use async_trait::async_trait;

use crate::{
    connector::{ConnectoPlugin, Plugins},
    cqrs::message_bus::Handler,
};

#[derive(Debug)]
pub struct QueryPlugins;

#[derive(Debug)]
pub struct QueryPluginsHandler {
    pub plugins: Plugins,
}

#[async_trait]
impl Handler for QueryPluginsHandler {
    type Message = QueryPlugins;

    type Output = Vec<&'static dyn ConnectoPlugin>;

    type Error = ();

    fn handle(&self, message: QueryPlugins) -> Result<Vec<&'static dyn ConnectoPlugin>, ()> {
        Ok(self.plugins.all())
    }
}
