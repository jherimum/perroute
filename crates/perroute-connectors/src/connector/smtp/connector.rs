use crate::plugin::{ConfigurationProperties, ConnectorPlugin, DispatcherPlugin};
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::dispatch_type::DispatcherType;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

use super::email_dispatcher::EmailDispatcher;

#[derive(Debug, Deserialize, Getters, Builder, Serialize)]
pub struct SmtpConnectorProperties {
    username: String,
    password: String,
    host: String,
    port: u16,
    timeout: Option<u64>,
}

#[derive(Debug)]
pub struct SmtpConnector {
    id: &'static str,
    configuration: ConfigurationProperties,
    plugins: HashMap<DispatcherType, Arc<dyn DispatcherPlugin>>,
}

impl Default for SmtpConnector {
    fn default() -> Self {
        let mut plugins: HashMap<DispatcherType, Arc<dyn DispatcherPlugin>> = HashMap::new();
        plugins.insert(DispatcherType::Email, Arc::new(EmailDispatcher::default()));
        Self {
            id: "smtp",
            configuration: ConfigurationProperties::default(),
            plugins,
        }
    }
}

impl ConnectorPlugin for SmtpConnector {
    fn id(&self) -> &str {
        self.id
    }

    fn configuration(&self) -> &ConfigurationProperties {
        &self.configuration
    }

    fn dispatchers(&self) -> HashMap<DispatcherType, Arc<dyn DispatcherPlugin>> {
        self.plugins
            .clone()
            .into_iter()
            .map(|(k, v)| (k, v.clone()))
            .collect()
    }
}
