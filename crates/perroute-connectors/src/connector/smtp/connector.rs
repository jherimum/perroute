use super::email_dispatcher::EmailDispatcher;
use crate::api::{
    BaseConnectorPlugin, ConfigurationProperties, ConfigurationPropertyBuilder,
    ConfigurationPropertyType, ConnectorPlugin, ConnectorPluginId, DispatchType, DispatcherPlugin,
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
pub struct SmtpConnector(BaseConnectorPlugin);

impl Default for SmtpConnector {
    fn default() -> Self {
        Self(BaseConnectorPlugin {
            plugin_id: todo!(),
            configuration: todo!(),
            dispatchers: todo!(),
        })
    }
}

impl ConnectorPlugin for SmtpConnector {
    fn id(&self) -> ConnectorPluginId {
        self.0.id()
    }

    fn configuration(&self) -> &ConfigurationProperties {
        &self.0.configuration()
    }

    fn dispatchers(&self) -> &HashMap<DispatchType, Box<dyn DispatcherPlugin>> {
        &self.0.dispatchers()
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
            .property_type(ConfigurationPropertyType::String)
            .multiple(false)
            .build()
            .unwrap(),
        ConfigurationPropertyBuilder::default()
            .name("password")
            .required(true)
            .description("SMTP password")
            .property_type(ConfigurationPropertyType::String)
            .multiple(false)
            .build()
            .unwrap(),
        ConfigurationPropertyBuilder::default()
            .name("port")
            .required(true)
            .description("SMTP port")
            .property_type(ConfigurationPropertyType::Number)
            .multiple(false)
            .build()
            .unwrap(),
        ConfigurationPropertyBuilder::default()
            .name("host")
            .required(true)
            .description("SMTP host")
            .property_type(ConfigurationPropertyType::String)
            .multiple(false)
            .build()
            .unwrap(),
        ConfigurationPropertyBuilder::default()
            .name("timeout")
            .required(false)
            .description("timeout in miliseconds")
            .property_type(ConfigurationPropertyType::Number)
            .multiple(false)
            .build()
            .unwrap(),
        ConfigurationPropertyBuilder::default()
            .name("starttls")
            .required(true)
            .description("starttls flag")
            .property_type(ConfigurationPropertyType::Boolean)
            .multiple(false)
            .build()
            .unwrap(),
    ]
    .into()
}
