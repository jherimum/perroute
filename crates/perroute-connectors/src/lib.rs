pub mod plugins;
pub mod types;

use perroute_commons::types::{
    id::Id,
    recipient::{EmailRecipient, PushRecipient, Recipient, SmsRecipient},
    template::{EmailTemplate, PushTemplate, SmsTemplate, Template},
    Configuration, ProviderId,
};
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, thiserror::Error)]
pub enum ProviderPluginError {
    #[error("Invalid data combination")]
    InvalidRequest,
}

pub fn repository() -> impl ProviderPluginRepository {
    DefaultProviderPluginRepository::default()
}

pub trait ProviderPluginRepository: Send + Sync {
    fn get(&self, id: &ProviderId) -> Option<&dyn ProviderPlugin>;
}

pub struct DefaultProviderPluginRepository {
    plugins: Arc<HashMap<ProviderId, Box<dyn ProviderPlugin>>>,
}

impl Default for DefaultProviderPluginRepository {
    fn default() -> Self {
        Self {
            plugins: Arc::new(HashMap::new()),
        }
    }
}

impl ProviderPluginRepository for DefaultProviderPluginRepository {
    fn get(&self, id: &ProviderId) -> Option<&dyn ProviderPlugin> {
        self.plugins.get(id).map(|plugin| plugin.as_ref())
    }
}

#[async_trait::async_trait]
pub trait ProviderPlugin: Send + Sync {
    async fn dispatch(
        &self,
        configuration: &Configuration,
        request: &DispatchRequest,
    ) -> Result<DispatchResponse, PluginDispatchError>;
}

#[derive(Debug, derive_more::From, Clone)]
pub enum DispatchRequest<'r> {
    Sms(Request<'r, SmsRecipient, SmsTemplate>),
    Email(Request<'r, EmailRecipient, EmailTemplate>),
    Push(Request<'r, PushRecipient, PushTemplate>),
}

impl<'r> DispatchRequest<'r> {
    pub fn id(&self) -> &Id {
        match self {
            DispatchRequest::Sms(request) => &request.id,
            DispatchRequest::Email(request) => &request.id,
            DispatchRequest::Push(request) => &request.id,
        }
    }

    pub fn create(
        id: &'r Id,
        recipient: &'r Recipient,
        template: &'r Template,
    ) -> Result<Self, ProviderPluginError> {
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

impl<'r> Request<'r, SmsRecipient, SmsTemplate> {
    pub fn sms(id: &'r Id, recipient: &'r SmsRecipient, template: &'r SmsTemplate) -> Self {
        Self {
            id,
            recipient,
            template,
        }
    }
}

impl<'r> Request<'r, EmailRecipient, EmailTemplate> {
    pub fn email(id: &'r Id, recipient: &'r EmailRecipient, template: &'r EmailTemplate) -> Self {
        Self {
            id,
            recipient,
            template,
        }
    }
}

impl<'r> Request<'r, PushRecipient, PushTemplate> {
    pub fn push(id: &'r Id, recipient: &'r PushRecipient, template: &'r PushTemplate) -> Self {
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
