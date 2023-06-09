use perroute_connectors::{ConfigurationProperty, DispatcherType};
use serde::Serialize;

#[derive(Serialize)]
pub struct ConnectorPluginResource {
    pub id: &'static str,
    pub properties: Vec<ConfigurationProperty>,
    pub dispatchers: Vec<Dispatcher>,
}

#[derive(Serialize)]
pub struct Dispatcher {
    pub type_: DispatcherType,
    pub properties: Vec<ConfigurationProperty>,
}
