use crate::{
    configuration::Configuration,
    template::DispatchTemplate,
    types::{ConnectorPluginId, DispatchType, TemplateSupport},
};
use derive_getters::Getters;
use erased_serde::serialize_trait_object;
use futures_util::future::BoxFuture;
use perroute_commons::types::{
    id::Id,
    payload::Payload,
    properties::Properties,
    recipient::Recipient,
    template::{TemplateData, TemplateError},
    vars::Vars,
};
use serde::Serialize;
use std::{error::Error, fmt::Debug};

pub trait ConnectorPlugin: Sync + Send + Debug {
    fn id(&self) -> ConnectorPluginId;
    fn configuration(&self) -> &dyn Configuration;
    fn dispatchers(&self) -> Vec<&dyn DispatcherPlugin>;
    fn dispatcher(&self, ty: &DispatchType) -> Option<&dyn DispatcherPlugin> {
        self.dispatchers()
            .iter()
            .find(|d| d.dispatch_type() == *ty)
            .copied()
    }
}

#[async_trait::async_trait]
pub trait DispatcherPlugin: Sync + Send + Debug {
    fn template_support(&self) -> TemplateSupport;
    fn dispatch_type(&self) -> DispatchType;
    fn configuration(&self) -> &dyn Configuration;
    async fn dispatch(&self, req: &DispatchRequest) -> Result<DispatchResponse, DispatchError>;
}

pub struct BaseDispatcherPlugin {
    pub dispatch_type: DispatchType,
    pub template_support: TemplateSupport,
    pub configuration: Box<dyn Configuration>,
    pub func:
        for<'r> fn(&'r DispatchRequest) -> BoxFuture<'r, Result<DispatchResponse, DispatchError>>,
}

impl BaseDispatcherPlugin {
    pub fn new(
        dispatch_type: DispatchType,
        template_support: TemplateSupport,
        configuration: Box<dyn Configuration>,
        func: for<'r> fn(
            &'r DispatchRequest,
        ) -> BoxFuture<'r, Result<DispatchResponse, DispatchError>>,
    ) -> Self {
        Self {
            dispatch_type,
            template_support,
            configuration,
            func,
        }
    }
}

impl Debug for BaseDispatcherPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BaseDispatcherPlugin")
            .field("dispatch_type", &self.dispatch_type)
            .field("template_support", &self.template_support)
            .field("configuration", &self.configuration)
            .finish()
    }
}

#[async_trait::async_trait]
impl DispatcherPlugin for BaseDispatcherPlugin {
    fn template_support(&self) -> TemplateSupport {
        self.template_support
    }

    fn dispatch_type(&self) -> DispatchType {
        self.dispatch_type
    }

    fn configuration(&self) -> &dyn Configuration {
        self.configuration.as_ref()
    }

    async fn dispatch(&self, req: &DispatchRequest) -> Result<DispatchResponse, DispatchError> {
        (self.func)(req).await
    }
}

#[derive(Getters)]
pub struct DispatchRequest<'r> {
    pub id: Id,
    pub connection_properties: &'r Properties,
    pub dispatch_properties: &'r Properties,
    pub template: Option<&'r dyn DispatchTemplate>,
    pub recipient: &'r Recipient,
    pub payload: &'r Payload,
    pub vars: &'r Vars,
    pub subject: Option<String>,
}

impl<'r> From<&DispatchRequest<'r>> for TemplateData {
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
    pub configuration: Box<dyn Configuration>,
    pub dispatchers: Vec<Box<dyn DispatcherPlugin>>,
}

impl ConnectorPlugin for BaseConnectorPlugin {
    fn id(&self) -> ConnectorPluginId {
        self.plugin_id
    }

    fn configuration(&self) -> &dyn Configuration {
        self.configuration.as_ref()
    }

    fn dispatchers(&self) -> Vec<&dyn DispatcherPlugin> {
        self.dispatchers.iter().map(AsRef::as_ref).collect()
    }
}

impl BaseConnectorPlugin {
    pub fn new(
        plugin_id: ConnectorPluginId,
        configuration: Box<dyn Configuration>,
        dispatchers: Vec<Box<dyn DispatcherPlugin>>,
    ) -> Self {
        Self {
            plugin_id,
            configuration,
            dispatchers,
        }
    }
}
