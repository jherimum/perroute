use api::{ConnectorPlugin, ConnectorPluginId};
use std::{collections::HashMap, fmt::Debug, sync::Arc};

pub mod api;
mod connector;

#[derive(Clone, Debug)]
pub struct Plugins {
    data: Arc<HashMap<ConnectorPluginId, Box<dyn ConnectorPlugin>>>,
}

impl Plugins {
    pub fn builder() -> PluginsBuilder {
        PluginsBuilder::default()
    }
    pub fn get(&self, id: &ConnectorPluginId) -> Option<&Box<dyn ConnectorPlugin>> {
        self.data.get(id)
    }

    pub fn all(&self) -> Vec<&Box<dyn ConnectorPlugin>> {
        self.data.values().collect::<Vec<_>>()
    }
}

#[derive(Debug, Default)]
pub struct PluginsBuilder {
    data: HashMap<ConnectorPluginId, Box<dyn ConnectorPlugin>>,
}

impl PluginsBuilder {
    pub fn with_plugin(mut self, plugin: Box<dyn ConnectorPlugin>) -> Self {
        self.data.insert(plugin.id(), plugin);
        self
    }

    pub fn build(self) -> Plugins {
        Plugins {
            data: Arc::new(self.data),
        }
    }
}
