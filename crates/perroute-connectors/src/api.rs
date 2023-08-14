use derive_builder::Builder;
use derive_getters::Getters;
use erased_serde::serialize_trait_object;
use perroute_commons::types::{
    id::Id,
    payload::Payload,
    properties::Properties,
    recipient::Recipient,
    template::{TemplateData, TemplateError},
    vars::Vars,
};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::{collections::HashMap, error::Error, fmt::Debug};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Type, Copy, Hash)]
#[sqlx(type_name = "dispatch_type", rename_all = "snake_case")]
pub enum DispatchType {
    Sms,
    Email,
    Push,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize, Type)]
pub enum ConnectorPluginId {
    Smtp,
    Log,
    SendGrid,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Serialize, Type)]
pub enum TemplateSupport {
    Mandatory,
    Optional,
    None,
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct OptionValue {}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub enum ConfigurationPropertyType {
    String,
    Number,
    Boolean,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Builder)]
pub struct ConfigurationProperty {
    name: &'static str,
    required: bool,
    description: &'static str,
    property_type: ConfigurationPropertyType,
    multiple: bool,
}

#[derive(Debug, Default)]
pub struct ConfigurationProperties(HashMap<&'static str, ConfigurationProperty>);

impl<const N: usize> From<[ConfigurationProperty; N]> for ConfigurationProperties {
    fn from(value: [ConfigurationProperty; N]) -> Self {
        ConfigurationProperties(
            value
                .into_iter()
                .map(|p| (p.name, p))
                .collect::<HashMap<_, _>>(),
        )
    }
}

pub trait ConnectorPlugin: Sync + Send + Debug {
    fn id(&self) -> ConnectorPluginId;
    fn configuration(&self) -> &ConfigurationProperties;
    fn dispatchers(&self) -> &HashMap<DispatchType, Box<dyn DispatcherPlugin>>;
    fn dispatcher(&self, ty: DispatchType) -> Option<&Box<dyn DispatcherPlugin>> {
        self.dispatchers().get(&ty)
    }
}

pub trait DispatchTemplate: Send + Sync {
    fn render_text(&self, data: &TemplateData) -> Result<Option<String>, TemplateError>;
    fn render_html(&self, data: &TemplateData) -> Result<Option<String>, TemplateError>;
}

#[derive(Getters)]
pub struct DispatchRequest<'t, 'p, 'v, 'r, 'cp, 'dp> {
    pub id: Id,
    pub connection_properties: &'cp Properties,
    pub dispatch_properties: &'dp Properties,
    pub template: Option<&'t dyn DispatchTemplate>,
    pub recipient: &'r Recipient,
    pub payload: &'p Payload,
    pub vars: &'v Vars,
    pub subject: Option<String>,
}

impl<'t, 'p, 'v, 'r, 'cp, 'dp> From<&DispatchRequest<'t, 'p, 'v, 'r, 'cp, 'dp>> for TemplateData {
    fn from(value: &DispatchRequest) -> Self {
        Self {
            payload: value.payload.clone(),
            recipient: value.recipient.clone(),
            vars: value.vars.clone(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct DispatchResponse {
    reference: Option<String>,
    data: Option<Box<dyn ResponseData>>,
}

impl DispatchResponse {
    pub fn new(reference: Option<String>, data: Option<Box<dyn ResponseData>>) -> Self {
        Self { reference, data }
    }
}

pub trait ResponseData: Debug + erased_serde::Serialize {}
serialize_trait_object!(ResponseData);

#[async_trait::async_trait]
pub trait DispatcherPlugin: Sync + Send + Debug {
    fn template_support(&self) -> TemplateSupport;
    fn dispatch_type(&self) -> DispatchType;
    fn configuration(&self) -> &ConfigurationProperties;
    async fn dispatch(&self, req: &DispatchRequest) -> Result<DispatchResponse, DispatchError>;
}

#[derive(Debug, thiserror::Error)]
pub enum DispatchError {
    #[error(transparent)]
    Recoverable(Box<dyn Error>),
    #[error(transparent)]
    Unrecoverable(Box<dyn Error>),
}

impl DispatchError {
    pub fn unrecoverable<E: Error + 'static>(e: E) -> Self {
        Self::Unrecoverable(Box::new(e))
    }
    pub fn recoverable<E: Error + 'static>(e: E) -> Self {
        Self::Recoverable(Box::new(e))
    }
}

impl From<TemplateError> for DispatchError {
    fn from(value: TemplateError) -> Self {
        Self::Unrecoverable(Box::new(value))
    }
}

#[derive(Debug)]
pub struct BaseConnectorPlugin {
    pub plugin_id: ConnectorPluginId,
    pub configuration: ConfigurationProperties,
    pub dispatchers: HashMap<DispatchType, Box<dyn DispatcherPlugin>>,
}

impl ConnectorPlugin for BaseConnectorPlugin {
    fn id(&self) -> ConnectorPluginId {
        self.plugin_id
    }

    fn configuration(&self) -> &ConfigurationProperties {
        &self.configuration
    }

    fn dispatchers(&self) -> &HashMap<DispatchType, Box<dyn DispatcherPlugin>> {
        &self.dispatchers
    }
}

impl BaseConnectorPlugin {
    pub fn new(
        plugin_id: ConnectorPluginId,
        configuration: ConfigurationProperties,
        dispatchers: HashMap<DispatchType, Box<dyn DispatcherPlugin>>,
    ) -> Self {
        Self {
            plugin_id,
            configuration,
            dispatchers,
        }
    }
}
