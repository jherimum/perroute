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
    plugins: Vec<Arc<dyn ConnectorPlugin>>,
}

impl Plugins {
    pub fn full() -> Self {
        Self {
            plugins: vec![
                Arc::new(smtp_connector_plugin()),
                Arc::new(log_connector_plugin()),
                Arc::new(sendgrid_connector_plugin()),
            ],
        }
    }
}

impl Plugins {
    pub fn get(&self, id: &ConnectorPluginId) -> Option<Arc<dyn ConnectorPlugin>> {
        self.plugins.iter().find(|p| p.id() == *id).cloned()
    }

    pub fn all(&self) -> Vec<Arc<dyn ConnectorPlugin>> {
        self.plugins.to_vec()
    }
}
