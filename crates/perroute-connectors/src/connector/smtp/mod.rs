use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use validator::Validate;

use self::email_dispatcher::email_dispatcher;
use crate::{
    api::{BaseConnectorPlugin, ConnectorPlugin},
    configuration::{
        ConfigurationProperties, ConfigurationPropertyBuilder, ConfigurationPropertyType,
        DefaultConfiguration,
    },
};
use std::{marker::PhantomData, sync::Arc};

mod connector;
mod email_dispatcher;

pub fn smtp_connector_plugin() -> impl ConnectorPlugin {
    BaseConnectorPlugin::new(
        crate::types::ConnectorPluginId::Smtp,
        Arc::new(DefaultConfiguration::new(
            smtp_conn_properties(),
            PhantomData::<SmtpConnectorProperties>,
        )),
        vec![Arc::new(email_dispatcher())],
    )
}

fn smtp_conn_properties() -> ConfigurationProperties {
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

#[derive(Debug, Deserialize, Getters, Builder, Serialize)]
pub struct SmtpConnectorProperties {
    username: String,
    password: String,
    host: String,
    port: u16,
    timeout: Option<u64>,
    starttls: bool,
}

impl Validate for SmtpConnectorProperties {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        todo!()
    }
}
