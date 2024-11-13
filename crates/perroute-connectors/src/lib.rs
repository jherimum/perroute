pub mod plugins;
pub mod types;

use perroute_commons::types::{
    recipient::{self, EmailRecipient, PushRecipient, SmsRecipient},
    template::{EmailTemplate, PushTemplate, SmsTemplate},
    Configuration, ProviderId,
};
use std::{collections::HashMap, future::Future, marker::PhantomData};

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
    fn email_dispatcher(
        &self,
        _: Configuration,
    ) -> Option<Box<dyn DispatcherTrait<EmailRecipient, EmailTemplate>>> {
        None
    }

    fn sms_dispatcher(
        &self,
        _: Configuration,
    ) -> Option<Box<dyn DispatcherTrait<SmsRecipient, SmsTemplate>>> {
        None
    }

    fn push_dispatcher(
        &self,
        _: Configuration,
    ) -> Option<Box<dyn DispatcherTrait<PushRecipient, PushTemplate>>> {
        None
    }
}

#[async_trait::async_trait]
pub trait DispatcherTrait<R, T> {
    async fn dispatch(&self, request: Request<R, T>) -> Result<DispatchResponse, DispatchError>;
}

#[async_trait::async_trait]
impl<R, T, F, O> DispatcherTrait<R, T> for Dispatcher<R, T, F, O>
where
    R: Sync + Send,
    T: Sync + Send,
    F: Fn(Configuration, R, T) -> O + Send + Sync,
    O: Future<Output = Result<DispatchResponse, DispatchError>> + Send + Sync,
{
    async fn dispatch(&self, request: Request<R, T>) -> Result<DispatchResponse, DispatchError> {
        let f = &self.function;

        f(
            self.configuration.clone(),
            request.recipient,
            request.template,
        )
        .await
    }
}

pub struct Dispatcher<R, T, F, O>
where
    F: Fn(Configuration, R, T) -> O,
    O: Future<Output = Result<DispatchResponse, DispatchError>>,
{
    pub function: F,
    configuration: Configuration,
    recipient: PhantomData<R>,
    template: PhantomData<T>,
}

impl<R, T, F, O> Dispatcher<R, T, F, O>
where
    F: Fn(Configuration, R, T) -> O,
    O: Future<Output = Result<DispatchResponse, DispatchError>>,
{
    pub fn new(configuration: Configuration, function: F) -> Self {
        Dispatcher {
            function,
            configuration,
            recipient: PhantomData,
            template: PhantomData,
        }
    }
}

pub struct Request<R, T> {
    recipient: R,
    template: T,
}

impl Request<EmailRecipient, EmailTemplate> {
    pub fn email(
        recipient: EmailRecipient,
        template: EmailTemplate,
    ) -> Request<EmailRecipient, EmailTemplate> {
        Request {
            recipient,
            template,
        }
    }
}

pub struct DispatchResponse {}

#[derive(Debug, thiserror::Error)]
pub enum DispatchError {}
