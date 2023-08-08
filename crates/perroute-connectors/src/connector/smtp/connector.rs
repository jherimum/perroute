use super::email_dispatcher::EmailDispatcher;
use crate::api::{
    ConfigurationProperties, ConnectorPlugin, ConnectorPluginId, DispatchType, DispatcherPlugin,
};
use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

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
    plugins: HashMap<DispatchType, Arc<dyn DispatcherPlugin>>,
}

impl Default for SmtpConnector {
    fn default() -> Self {
        let mut plugins: HashMap<DispatchType, Arc<dyn DispatcherPlugin>> = HashMap::new();
        plugins.insert(DispatchType::Email, Arc::new(EmailDispatcher::default()));
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

    fn dispatchers(&self) -> HashMap<DispatchType, Arc<dyn DispatcherPlugin>> {
        self.plugins
            .clone()
            .into_iter()
            .map(|(k, v)| (k, v.clone()))
            .collect()
    }
}
