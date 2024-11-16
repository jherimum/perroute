pub mod plugins;
pub mod types;

use perroute_commons::types::{
    id::Id,
    recipient::{EmailRecipient, PushRecipient, Recipient, SmsRecipient},
    template::{EmailTemplate, PushTemplate, SmsTemplate, Template},
    Configuration, ProviderId,
};
use std::collections::HashMap;

pub fn repository() -> impl ProviderPluginRepository {
    DefaultProviderPluginRepository {
        plugins: HashMap::new(),
    }
}

pub trait ProviderPluginRepository {
    fn get(&self, id: &ProviderId) -> Option<&dyn ProviderPlugin>;
}

pub struct DefaultProviderPluginRepository {
    plugins: HashMap<ProviderId, Box<dyn ProviderPlugin>>,
}

impl ProviderPluginRepository for DefaultProviderPluginRepository {
    fn get(&self, id: &ProviderId) -> Option<&dyn ProviderPlugin> {
        self.plugins.get(id).map(|plugin| plugin.as_ref())
    }
}

#[async_trait::async_trait]
pub trait ProviderPlugin {
    async fn dispatch(
        &self,
        configuration: &Configuration,
        request: &DispatchRequest,
    ) -> Result<DispatchResponse, PluginDispatchError>;
}

#[derive(Debug, derive_more::From)]
pub enum DispatchRequest {
    Sms(Request<SmsRecipient, SmsTemplate>),
    Email(Request<EmailRecipient, EmailTemplate>),
    Push(Request<PushRecipient, PushTemplate>),
}

impl DispatchRequest {
    pub fn id(&self) -> &Id {
        match self {
            DispatchRequest::Sms(request) => &request.id,
            DispatchRequest::Email(request) => &request.id,
            DispatchRequest::Push(request) => &request.id,
        }
    }
}

impl TryFrom<(&Id, Recipient, Template)> for DispatchRequest {
    type Error = &'static str;

    fn try_from(
        (id, recipient, template): (&Id, Recipient, Template),
    ) -> Result<Self, Self::Error> {
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
            _ => Err("Invalid recipient and template combination"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Request<R, T> {
    id: Id,
    recipient: R,
    template: T,
}

impl Request<SmsRecipient, SmsTemplate> {
    pub fn sms(id: &Id, recipient: SmsRecipient, template: SmsTemplate) -> Self {
        Self {
            id: id.clone(),
            recipient,
            template,
        }
    }
}

impl Request<EmailRecipient, EmailTemplate> {
    pub fn email(id: &Id, recipient: EmailRecipient, template: EmailTemplate) -> Self {
        Self {
            id: id.clone(),
            recipient,
            template,
        }
    }
}

impl Request<PushRecipient, PushTemplate> {
    pub fn push(id: &Id, recipient: PushRecipient, template: PushTemplate) -> Self {
        Self {
            id: id.clone(),
            recipient,
            template,
        }
    }
}

pub struct DispatchResponse {}

#[derive(Debug, thiserror::Error)]
pub enum PluginDispatchError {}
