use plugin::ConnectorPlugin;
use std::{collections::HashMap, fmt::Debug, sync::Arc};

mod connector;
pub mod plugin;

#[derive(Clone, Debug)]
pub struct Plugins {
    data: Arc<HashMap<String, Arc<dyn ConnectorPlugin>>>,
}

impl Plugins {
    pub fn builder() -> PluginsBuilder {
        PluginsBuilder::default()
    }
    pub fn get(&self, id: &str) -> Option<Arc<dyn ConnectorPlugin>> {
        self.data.get(id).cloned()
    }

    pub fn all(&self) -> Vec<Arc<dyn ConnectorPlugin>> {
        self.data.values().cloned().collect::<Vec<_>>()
    }
}

#[derive(Debug, Default)]
pub struct PluginsBuilder {
    data: HashMap<String, Arc<dyn ConnectorPlugin>>,
}

impl PluginsBuilder {
    pub fn with_plugin(mut self, plugin: Arc<dyn ConnectorPlugin>) -> Self {
        self.data.insert(plugin.id().to_owned(), plugin);
        self
    }

    pub fn build(self) -> Plugins {
        Plugins {
            data: Arc::new(self.data),
        }
    }
}
