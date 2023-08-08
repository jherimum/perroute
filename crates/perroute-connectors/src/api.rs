use derive_getters::Getters;
use erased_serde::serialize_trait_object;
use perroute_commons::types::{
    dispatch_type::DispatchType,
    id::Id,
    payload::Payload,
    properties::Properties,
    recipient::Recipient,
    template::{TemplateData, TemplateError},
    vars::Vars,
};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::{
    collections::HashMap,
    error::Error,
    fmt::{Debug, Display},
    sync::Arc,
};

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Serialize, Type)]
pub enum TemplateSupport {
    Mandatory,
    Optional,
    None,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Serialize)]
pub enum ConfigurationPropertyType {
    String,
    Number,
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct OptionValue {}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct ConfigurationProperty {
    pub name: String,
    pub required: bool,
    pub description: String,
    pub possible_values: Vec<OptionValue>,
    pub type_: ConfigurationPropertyType,
}

#[derive(Debug, Default)]
pub struct ConfigurationProperties {
    pub properties: Vec<ConfigurationProperty>,
}

pub trait ConnectorPlugin: Sync + Send + Debug {
    fn id(&self) -> &str;
    fn configuration(&self) -> &ConfigurationProperties;
    fn dispatchers(&self) -> HashMap<DispatchType, Arc<dyn DispatcherPlugin>>;
    fn dispatcher(&self, ty: DispatchType) -> Option<Arc<dyn DispatcherPlugin>> {
        self.dispatchers().get(&ty).map(Arc::clone)
    }
}

pub trait DispatchTemplate {
    fn render_subject(&self, data: &TemplateData) -> Result<Option<String>, TemplateError>;
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

pub trait DispatcherPlugin: Sync + Send + Debug {
    fn template_support(&self) -> TemplateSupport;
    fn dispatch_type(&self) -> DispatchType;
    fn configuration(&self) -> &ConfigurationProperties;
    fn dispatch(&self, req: &DispatchRequest) -> Result<DispatchResponse, DispatchError>;
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
