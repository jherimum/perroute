use crate::{
    configuration::Configuration,
    template::DispatchTemplate,
    types::{delivery::Delivery, dispatch_type::DispatchType, plugin_id::ConnectorPluginId},
};
use derive_getters::Getters;
use erased_serde::serialize_trait_object;
use futures_util::future::BoxFuture;
use perroute_commons::types::{
    id::Id,
    payload::Payload,
    properties::Properties,
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
    fn dispatch_type(&self) -> DispatchType;
    fn configuration(&self) -> &dyn Configuration;
    async fn dispatch(&self, req: &DispatchRequest) -> Result<DispatchResponse, DispatchError>;
}

pub struct BaseDispatcherPlugin {
    pub dispatch_type: DispatchType,
    pub configuration: Box<dyn Configuration>,
    pub func:
        for<'r> fn(&'r DispatchRequest) -> BoxFuture<'r, Result<DispatchResponse, DispatchError>>,
}

impl BaseDispatcherPlugin {
    pub fn new(
        dispatch_type: DispatchType,
        configuration: Box<dyn Configuration>,
        func: for<'r> fn(
            &'r DispatchRequest,
        ) -> BoxFuture<'r, Result<DispatchResponse, DispatchError>>,
    ) -> Self {
        Self {
            dispatch_type,
            configuration,
            func,
        }
    }
}

impl Debug for BaseDispatcherPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BaseDispatcherPlugin")
            .field("dispatch_type", &self.dispatch_type)
            .field("configuration", &self.configuration)
            .finish()
    }
}

#[async_trait::async_trait]
impl DispatcherPlugin for BaseDispatcherPlugin {
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
    pub template: &'r dyn DispatchTemplate,
    pub payload: &'r Payload,
    pub vars: &'r Vars,
    pub delivery: Delivery,
}

impl<'r> From<&DispatchRequest<'r>> for TemplateData {
    fn from(value: &DispatchRequest) -> Self {
        Self {
            payload: value.payload.clone(),
            vars: value.vars.clone(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct DispatchResponse {
    pub reference: Option<String>,
    pub data: Option<Box<dyn ResponseData>>,
}

pub trait ResponseData: Debug + erased_serde::Serialize + Send + Sync {}
serialize_trait_object!(ResponseData);

#[derive(Debug, thiserror::Error)]
pub enum DispatchError {
    #[error(transparent)]
    Recoverable(Box<dyn Error + Send + Sync>),
    #[error(transparent)]
    Unrecoverable(Box<dyn Error + Send + Sync>),
}

impl DispatchError {
    pub fn unrecoverable<E: Error + Send + Sync + 'static>(e: E) -> Self {
        Self::Unrecoverable(Box::new(e))
    }
    pub fn recoverable<E: Error + Send + Sync + 'static>(e: E) -> Self {
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
