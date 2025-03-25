use crate::{DispatchResponse, PluginDispatchError};
use futures::future::BoxFuture;
use perroute_commons::types::{
    dispatch_type::DispatchType,
    recipient::{EmailRecipient, PushRecipient, SmsRecipient},
    Configuration,
};
use perroute_template::template::{
    EmailTemplate, PushTemplate, RenderedTemplateState, SmsTemplate,
};
use std::{collections::HashMap, future::Future};

pub struct ProvidePlugin {
    sms: Option<
        DispatcherPlugin<SmsRecipient, SmsTemplate<RenderedTemplateState>>,
    >,
    email: Option<
        DispatcherPlugin<EmailRecipient, EmailTemplate<RenderedTemplateState>>,
    >,
    push: Option<
        DispatcherPlugin<PushRecipient, PushTemplate<RenderedTemplateState>>,
    >,
}

impl ProvidePlugin {
    pub fn properties(
        &self,
    ) -> HashMap<DispatchType, &HashMap<String, String>> {
        let mut properties = HashMap::new();
        if let Some(sms) = &self.sms {
            properties.insert(DispatchType::Sms, &sms.properties);
        }
        if let Some(email) = &self.email {
            properties.insert(DispatchType::Email, &email.properties);
        }
        if let Some(push) = &self.push {
            properties.insert(DispatchType::Push, &push.properties);
        }

        properties
    }

    pub fn sms<'a>(
        &'a self,
        cfg: &'a Configuration,
    ) -> Option<Dispatcher<'a, SmsRecipient, SmsTemplate<RenderedTemplateState>>>
    {
        self.sms.as_ref().map(|d| Dispatcher {
            cfg,
            func: d.send.as_ref(),
        })
    }

    pub fn push<'a>(
        &'a self,
        cfg: &'a Configuration,
    ) -> Option<
        Dispatcher<'a, PushRecipient, PushTemplate<RenderedTemplateState>>,
    > {
        self.push.as_ref().map(|d| Dispatcher {
            cfg,
            func: d.send.as_ref(),
        })
    }

    pub fn email<'a>(
        &'a self,
        cfg: &'a Configuration,
    ) -> Option<
        Dispatcher<'a, EmailRecipient, EmailTemplate<RenderedTemplateState>>,
    > {
        self.email.as_ref().map(|d| Dispatcher {
            cfg,
            func: d.send.as_ref(),
        })
    }
}

pub struct Dispatcher<'d, R, T> {
    cfg: &'d Configuration,
    func: &'d dyn DispatchFunction<R, T>,
}

impl<R, T> Dispatcher<'_, R, T> {
    async fn execute(
        &self,
        recipient: R,
        template: T,
    ) -> Result<DispatchResponse, PluginDispatchError> {
        self.func
            .dispatch(self.cfg.clone(), recipient, template)
            .await
    }
}

pub struct DispatcherPlugin<R, T> {
    send: Box<dyn DispatchFunction<R, T>>,
    properties: HashMap<String, String>,
}

impl<R, T> DispatcherPlugin<R, T> {
    fn new(
        properties: HashMap<String, String>,
        func: impl DispatchFunction<R, T> + 'static,
    ) -> Self {
        DispatcherPlugin {
            send: Box::new(func),
            properties,
        }
    }
}

pub trait DispatchFunction<R, T> {
    fn dispatch(
        &self,
        configuration: Configuration,
        recipient: R,
        template: T,
    ) -> BoxFuture<'static, Result<DispatchResponse, PluginDispatchError>>;
}

impl<F, R, T, O> DispatchFunction<R, T> for F
where
    F: Fn(Configuration, R, T) -> O,
    O: Future<Output = Result<DispatchResponse, PluginDispatchError>>
        + 'static
        + Send
        + Sync,
{
    fn dispatch(
        &self,
        configuration: Configuration,
        recipient: R,
        template: T,
    ) -> BoxFuture<'static, Result<DispatchResponse, PluginDispatchError>> {
        Box::pin((self)(configuration, recipient, template))
    }
}
