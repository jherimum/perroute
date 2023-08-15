use api::ConnectorPlugin;
use std::{collections::HashMap, fmt::Debug, sync::Arc};
use types::ConnectorPluginId;

pub mod api;
pub mod configuration;
pub mod connector;
pub mod template;
pub mod types;

use connector::smtp::smtp_connector_plugin;

#[derive(Clone, Debug)]
pub struct Plugins {
    plugins: Arc<Vec<Box<dyn ConnectorPlugin>>>,
}

impl Plugins {
    pub fn full() -> Self {
        Self {
            plugins: Arc::new(vec![Box::new(smtp_connector_plugin())]),
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
