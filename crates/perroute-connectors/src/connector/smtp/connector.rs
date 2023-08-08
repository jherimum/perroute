use super::email_dispatcher::EmailDispatcher;
use crate::api::{
    ConfigurationProperties, ConnectorPlugin, ConnectorPluginId, DispatchType, DispatcherPlugin,
};
use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Getters, Builder, Serialize)]
pub struct SmtpConnectorProperties {
    username: String,
    password: String,
    host: String,
    port: u16,
    timeout: Option<u64>,
    starttls: bool,
}

#[derive(Debug)]
pub struct SmtpConnector {
    id: ConnectorPluginId,
    configuration: ConfigurationProperties,
    plugins: HashMap<DispatchType, Box<dyn DispatcherPlugin>>,
}

impl Default for SmtpConnector {
    fn default() -> Self {
        let mut plugins: HashMap<DispatchType, Box<dyn DispatcherPlugin>> = HashMap::new();
        plugins.insert(DispatchType::Email, Box::new(EmailDispatcher::default()));
        Self {
            id: ConnectorPluginId::Smtp,
            configuration: ConfigurationProperties::default(),
            plugins,
        }
    }
}

impl ConnectorPlugin for SmtpConnector {
    fn id(&self) -> ConnectorPluginId {
        self.id
    }

    fn configuration(&self) -> &ConfigurationProperties {
        &self.configuration
    }

    fn dispatchers(&self) -> &HashMap<DispatchType, Box<dyn DispatcherPlugin>> {
        &self.plugins
    }
}
