//pub mod concrete_plugin;
pub mod generic_plugins;
pub mod plugins;
pub mod spi;
pub mod types;

use perroute_commons::types::{
    id::Id,
    recipient::{EmailRecipient, PushRecipient, Recipient, SmsRecipient},
    Configuration, ProviderId,
};
use perroute_template::template::{
    EmailTemplate, PushTemplate, RenderedTemplateState, SmsTemplate, Template,
};
use std::{collections::HashMap, fmt::Debug, sync::Arc};

#[derive(Debug, thiserror::Error)]
pub enum ProviderPluginError {
    #[error("Invalid data combination")]
    InvalidRequest,
}

// pub fn new_repository() -> PluginRepository {
//     PluginRepository::default().add(LogPovider)
// }

pub fn plugin_repository() -> ProviderPluginRepository {
    ProviderPluginRepository::default()
}

#[derive(Clone, Debug)]
pub struct ProviderPluginRepository {
    plugins: Arc<HashMap<ProviderId, Arc<dyn ProviderPlugin>>>,
}

impl Default for ProviderPluginRepository {
    fn default() -> Self {
        Self {
            plugins: Arc::new(HashMap::new()),
        }
    }
}

impl ProviderPluginRepository {
    pub fn get(&self, id: &ProviderId) -> Option<&Arc<dyn ProviderPlugin>> {
        self.plugins.get(id)
    }

    pub fn add(mut self, plugin: Arc<dyn ProviderPlugin>) {}
}

#[async_trait::async_trait]
pub trait ProviderPlugin: Send + Sync + Debug {
    fn id(&self) -> ProviderId;

    async fn dispatch(
        &self,
        configuration: &Configuration,
        request: &DispatchRequest,
    ) -> Result<DispatchResponse, PluginDispatchError>;
}

#[derive(Debug, derive_more::From, Clone)]
pub enum DispatchRequest<'r> {
    Sms(Request<'r, SmsRecipient, SmsTemplate<RenderedTemplateState>>),
    Email(Request<'r, EmailRecipient, EmailTemplate<RenderedTemplateState>>),
    Push(Request<'r, PushRecipient, PushTemplate<RenderedTemplateState>>),
}

impl DispatchRequest<'_> {
    pub fn id(&self) -> &Id {
        match self {
            DispatchRequest::Sms(request) => &request.id,
            DispatchRequest::Email(request) => &request.id,
            DispatchRequest::Push(request) => &request.id,
        }
    }

    pub fn create<'r>(
        id: &'r Id,
        recipient: &'r Recipient,
        template: &'r Template<RenderedTemplateState>,
    ) -> Result<DispatchRequest<'r>, ProviderPluginError> {
        match (id, recipient, template) {
            (id, Recipient::Sms(recipient), Template::Sms(template)) => {
                Ok(DispatchRequest::Sms(Request::sms(id, recipient, template)))
            }
            (id, Recipient::Email(recipient), Template::Email(template)) => Ok(
                DispatchRequest::Email(Request::email(id, recipient, template)),
            ),
            (id, Recipient::Push(recipient), Template::Push(template)) => Ok(
                DispatchRequest::Push(Request::push(id, recipient, template)),
            ),
            _ => Err(ProviderPluginError::InvalidRequest),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Request<'r, R, T> {
    id: &'r Id,
    recipient: &'r R,
    template: &'r T,
}

impl<'r> Request<'r, SmsRecipient, SmsTemplate<RenderedTemplateState>> {
    pub fn sms(
        id: &'r Id,
        recipient: &'r SmsRecipient,
        template: &'r SmsTemplate<RenderedTemplateState>,
    ) -> Self {
        Self {
            id,
            recipient,
            template,
        }
    }
}

impl<'r> Request<'r, EmailRecipient, EmailTemplate<RenderedTemplateState>> {
    pub fn email(
        id: &'r Id,
        recipient: &'r EmailRecipient,
        template: &'r EmailTemplate<RenderedTemplateState>,
    ) -> Self {
        Self {
            id,
            recipient,
            template,
        }
    }
}

impl<'r> Request<'r, PushRecipient, PushTemplate<RenderedTemplateState>> {
    pub fn push(
        id: &'r Id,
        recipient: &'r PushRecipient,
        template: &'r PushTemplate<RenderedTemplateState>,
    ) -> Self {
        Self {
            id,
            recipient,
            template,
        }
    }
}

pub struct DispatchResponse {}

#[derive(Debug, thiserror::Error)]
pub enum PluginDispatchError {}
