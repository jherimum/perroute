use super::email_dispatcher::EmailDispatcher;
use crate::api::{
    ConfigurationProperties, ConfigurationPropertyBuilder, ConnectorPlugin, ConnectorPluginId,
    DispatchType, DispatcherPlugin,
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
        Self {
            id: ConnectorPluginId::Smtp,
            configuration: properties(),
            plugins: dispatchers(),
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

fn dispatchers() -> HashMap<DispatchType, Box<dyn DispatcherPlugin>> {
    let mut plugins: HashMap<DispatchType, Box<dyn DispatcherPlugin>> = HashMap::new();
    plugins.insert(DispatchType::Email, Box::new(EmailDispatcher::default()));
    plugins
}

fn properties() -> ConfigurationProperties {
    [
        ConfigurationPropertyBuilder::default()
            .name("username")
            .required(true)
            .description("SMTP username")
            .build()
            .unwrap(),
        ConfigurationPropertyBuilder::default()
            .name("password")
            .required(true)
            .description("SMTP password")
            .build()
            .unwrap(),
        ConfigurationPropertyBuilder::default()
            .name("port")
            .required(true)
            .description("SMTP port")
            .build()
            .unwrap(),
        ConfigurationPropertyBuilder::default()
            .name("host")
            .required(true)
            .description("SMTP host")
            .build()
            .unwrap(),
        ConfigurationPropertyBuilder::default()
            .name("timeout")
            .required(false)
            .description("timeout in miliseconds")
            .build()
            .unwrap(),
        ConfigurationPropertyBuilder::default()
            .name("starttls")
            .required(true)
            .description("starttls flag")
            .build()
            .unwrap(),
    ]
    .into()
}
