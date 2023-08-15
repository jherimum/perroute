use api::ConnectorPlugin;
use connector::{
    log::log_connector_plugin, sendgrid::sendgrid_connector_plugin, smtp::smtp_connector_plugin,
};
use std::{fmt::Debug, sync::Arc};
use types::ConnectorPluginId;

pub mod api;
pub mod configuration;
mod connector;
pub mod template;
pub mod types;

#[derive(Clone, Debug)]
pub struct Plugins {
    plugins: Arc<Vec<Box<dyn ConnectorPlugin>>>,
}

impl Plugins {
    pub fn full() -> Self {
        Self {
            plugins: Arc::new(vec![
                Box::new(smtp_connector_plugin()),
                Box::new(log_connector_plugin()),
                Box::new(sendgrid_connector_plugin()),
            ]),
        }
    }
}

impl Plugins {
    pub fn get(&self, id: ConnectorPluginId) -> Option<&dyn ConnectorPlugin> {
        self.plugins
            .iter()
            .find(|p| p.id() == id)
            .map(|p| p.as_ref())
    }

    pub fn all(&self) -> Vec<&dyn ConnectorPlugin> {
        self.plugins.iter().map(|p| p.as_ref()).collect()
    }
}
