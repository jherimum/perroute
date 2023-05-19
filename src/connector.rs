use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug, sync::Arc};
mod smtp;

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Serialize, Hash)]
pub enum DispatcherType {
    SMS,
    EMAIL,
    PUSH,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Serialize)]
pub enum ConfigurationPropertyType {
    String,
    Integer,
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct OptionValue {}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct ConfigurationProperty {
    pub name: String,
    pub required: bool,
    pub description: String,
    pub possible_values: Vec<OptionValue>,
    pub type_: ConfigurationPropertyType,
}

pub struct Configuration<'p> {
    pub properties: &'p Vec<ConfigurationProperty>,
}

pub trait ConnectoPlugin: Sync + Send + Debug {
    fn id(&self) -> &'static str;
    fn configuration(&self) -> Configuration;
    fn dispatchers(&self) -> HashMap<DispatcherType, &'static dyn DispatcherPlugin>;
}

pub trait DispatcherPlugin: Sync + Send + Debug {
    fn type_(&self) -> DispatcherType;
    fn configuration(&self) -> &Configuration;
}

#[derive(Clone, Debug)]
pub struct Plugins {
    data: Arc<HashMap<&'static str, &'static dyn ConnectoPlugin>>,
}

impl Plugins {
    pub fn builder() -> PluginsBuilder {
        PluginsBuilder::default()
    }
    pub fn get(&self, id: &str) -> Option<&'static dyn ConnectoPlugin> {
        self.data.get(id).copied()
    }

    pub fn all(&self) -> Vec<&'static dyn ConnectoPlugin> {
        self.data.values().copied().to_owned().collect::<Vec<_>>()
    }
}

#[derive(Debug, Default)]
pub struct PluginsBuilder {
    data: HashMap<&'static str, &'static dyn ConnectoPlugin>,
}

impl PluginsBuilder {
    pub fn with_plugin(mut self, plugin: &'static dyn ConnectoPlugin) -> Self {
        self.data.insert(plugin.id(), plugin);
        self
    }

    pub fn build(self) -> Plugins {
        Plugins {
            data: Arc::new(self.data),
        }
    }
}
