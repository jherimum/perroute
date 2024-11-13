pub mod plugins;
pub mod types;

use perroute_commons::types::{
    recipient::{EmailRecipient, PushRecipient, SmsRecipient},
    template::{EmailTemplate, PushTemplate, SmsTemplate},
    Configuration, ProviderId,
};
use std::{collections::HashMap, future::Future};

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
        configuration: Configuration,
        request: DispatchRequest,
    ) -> Result<DispatchResponse, DispatchError>;
}

pub struct DefaulPlugin<O>
where
    O: Future<Output = Result<DispatchResponse, DispatchError>>,
{
    id: ProviderId,
    sms: Option<Dispatcher<SmsRecipient, SmsTemplate, O>>,
    email: Option<Dispatcher<EmailRecipient, EmailTemplate, O>>,
    push: Option<Dispatcher<PushRecipient, PushTemplate, O>>,
}

#[async_trait::async_trait]
impl<O: Future<Output = Result<DispatchResponse, DispatchError>> + Send + Sync> ProviderPlugin
    for DefaulPlugin<O>
where
    Self: Send + Sync,
{
    async fn dispatch(
        &self,
        configuration: Configuration,
        request: DispatchRequest,
    ) -> Result<DispatchResponse, DispatchError> {
        match request {
            DispatchRequest::Email(request) => {
                let x = self.email.as_ref().unwrap();
                x.dispatch(configuration, request.recipient, request.template)
                    .await
            }
            DispatchRequest::Sms(request) => {
                let x = self.sms.as_ref().unwrap();
                x.dispatch(configuration, request.recipient, request.template)
                    .await
            }
            DispatchRequest::Push(request) => {
                let x = self.push.as_ref().unwrap();
                x.dispatch(configuration, request.recipient, request.template)
                    .await
            }
        }
    }
}

pub struct Dispatcher<R, T, O>
where
    O: Future<Output = Result<DispatchResponse, DispatchError>>,
{
    function: fn(Configuration, R, T) -> O,
}

impl<R, T, O> Dispatcher<R, T, O>
where
    O: Future<Output = Result<DispatchResponse, DispatchError>>,
{
    pub fn new(function: fn(Configuration, R, T) -> O) -> Self {
        Dispatcher { function }
    }

    fn dispatch(&self, configuration: Configuration, recipient: R, template: T) -> O {
        (self.function)(configuration, recipient, template)
    }
}

pub enum DispatchRequest {
    Sms(Request<SmsRecipient, SmsTemplate>),
    Email(Request<EmailRecipient, EmailTemplate>),
    Push(Request<PushRecipient, PushTemplate>),
}

pub struct Request<R, T> {
    recipient: R,
    template: T,
}

pub struct DispatchResponse {}

#[derive(Debug, thiserror::Error)]
pub enum DispatchError {}
