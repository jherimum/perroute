use std::{collections::HashMap, future::Future, pin::Pin};
use perroute_commons::types::{
    dispatch_type::DispatchType,
    id::Id,
    recipient::{EmailRecipient, PushRecipient, SmsRecipient},
    Configuration,
};
use perroute_template::template::{
    EmailTemplate, PushTemplate, RenderedTemplateState, SmsTemplate,
};
use crate::types::Properties;

pub type SmsCapbility =
    Capability<SmsRecipient, SmsTemplate<RenderedTemplateState>>;
pub type EmailCapbility =
    Capability<EmailRecipient, EmailTemplate<RenderedTemplateState>>;
pub type PushCapbility =
    Capability<PushRecipient, PushTemplate<RenderedTemplateState>>;

#[async_trait::async_trait]
pub trait ProviderPluginTrait {
    fn id(&self) -> &str;
    fn supported_dispatch_types(&self) -> Vec<DispatchType>;
    async fn dispatch(
        &self,
        cfg: &Configuration,
        request: &DispatchRequest<'_>,
    ) -> Result<Response, Error>;
}

pub struct Dispatcher<'d, R, T> {
    cfg: &'d Configuration,
    function: &'d DispatchFunction<R, T>,
}

type DispatchFunctionResult<'a> =
    Pin<Box<dyn Future<Output = Result<Response, Error>> + 'a + Send + Sync>>;

type DispatchFunction<R, T> = Box<
    dyn for<'a> Fn(
            &'a Configuration,
            &'a Request<R, T>,
        ) -> DispatchFunctionResult<'a>
        + Send
        + Sync,
>;

impl<'d, R, T> Dispatcher<'d, R, T>
where
    R: Clone,
    T: Clone,
{
    pub async fn dispatch(
        &self,
        request: &Request<'_, R, T>,
    ) -> Result<Response, Error> {
        (self.function)(&self.cfg, &request).await
    }
}

pub struct Response;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("")]
    ValidationError,
}

pub enum DispatchRequest<'r> {
    Email(Request<'r, EmailRecipient, EmailTemplate<RenderedTemplateState>>),
    Sms(Request<'r, SmsRecipient, SmsTemplate<RenderedTemplateState>>),
    Push(Request<'r, PushRecipient, PushTemplate<RenderedTemplateState>>),
}

#[derive(Clone)]
pub struct Request<'r, R, T> {
    message_id: &'r Id,
    recipient: &'r R,
    template: &'r T,
}

pub struct Capability<R, T> {
    properties: Properties,
    function: DispatchFunction<R, T>,
}

impl<R: Send + Sync, T: Send + Sync> Capability<R, T> {
    pub fn dispatcher<'a>(
        &'a self,
        cfg: &'a Configuration,
    ) -> Result<Dispatcher<'a, R, T>, Error> {
        self.validate_configuration(cfg)
            .map_err(|e| Error::ValidationError)?;
        Ok(Dispatcher {
            cfg,
            function: &self.function,
        })
    }
    pub fn validate_configuration(
        &self,
        cfg: &Configuration,
    ) -> Result<(), Vec<String>> {
        self.properties.validate(cfg)
    }
}

pub struct ProviderPlugin {
    id: String,
    email: Option<EmailCapbility>,
    sms: Option<SmsCapbility>,
    push: Option<PushCapbility>,
}

#[async_trait::async_trait]
impl ProviderPluginTrait for ProviderPlugin {
    fn id(&self) -> &str {
        &self.id
    }

    fn supported_dispatch_types(&self) -> Vec<DispatchType> {
        let mut dispatch_types = vec![];
        if self.email.is_some() {
            dispatch_types.push(DispatchType::Email);
        }

        if self.sms.is_some() {
            dispatch_types.push(DispatchType::Sms);
        }

        if self.push.is_some() {
            dispatch_types.push(DispatchType::Push);
        }

        dispatch_types
    }

    async fn dispatch(
        &self,
        cfg: &Configuration,
        request: &DispatchRequest<'_>,
    ) -> Result<Response, Error> {
        match request {
            DispatchRequest::Email(request) => match &self.email {
                Some(c) => c.dispatcher(cfg)?.dispatch(request).await,
                None => todo!(),
            },
            DispatchRequest::Sms(request) => match &self.sms {
                Some(c) => c.dispatcher(cfg)?.dispatch(request).await,
                None => todo!(),
            },
            DispatchRequest::Push(request) => match &self.push {
                Some(c) => c.dispatcher(cfg)?.dispatch(request).await,
                None => todo!(),
            },
        }
    }
}

impl ProviderPlugin {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_owned(),
            email: None,
            sms: None,
            push: None,
        }
    }

    pub fn with_email(
        mut self,
        properties: Properties,
        function: DispatchFunction<
            EmailRecipient,
            EmailTemplate<RenderedTemplateState>,
        >,
    ) -> Self {
        self.email = Some(Capability {
            properties,
            function,
        });
        self
    }

    pub fn with_sms(
        mut self,
        properties: Properties,
        function: DispatchFunction<
            SmsRecipient,
            SmsTemplate<RenderedTemplateState>,
        >,
    ) -> Self {
        self.sms = Some(Capability {
            properties,
            function,
        });
        self
    }

    pub fn with_push(
        mut self,
        properties: Properties,
        function: DispatchFunction<
            PushRecipient,
            PushTemplate<RenderedTemplateState>,
        >,
    ) -> Self {
        self.push = Some(Capability {
            properties,
            function,
        });
        self
    }

    fn id(&self) -> &str {
        &self.id
    }

    pub fn email_capability(&self) -> Option<&EmailCapbility> {
        self.email.as_ref()
    }

    pub fn push_capability(&self) -> Option<&PushCapbility> {
        self.push.as_ref()
    }

    pub fn sms_capability(&self) -> Option<&SmsCapbility> {
        self.sms.as_ref()
    }
}

#[derive(Default)]
pub struct PluginRepository {
    plugins: HashMap<String, Box<dyn ProviderPluginTrait>>,
}

impl PluginRepository {
    pub fn add_plugin(
        mut self,
        plugin: impl Into<Box<dyn ProviderPluginTrait>>,
    ) -> Self {
        let plugin = plugin.into();
        self.plugins.insert(plugin.id().to_owned(), plugin);
        self
    }

    pub fn get(&self, plugin_id: &str) -> Option<&dyn ProviderPluginTrait> {
        self.plugins
            .get(&String::from(plugin_id))
            .map(AsRef::as_ref)
    }

    pub fn all(&self) -> Vec<&dyn ProviderPluginTrait> {
        self.plugins.values().map(AsRef::as_ref).collect()
    }
}

#[cfg(test)]
mod tests {

    use perroute_commons::types::Configuration;
    use crate::plugins::smtp::SmtpProvider;
    use super::PluginRepository;

    #[test]
    fn name() {
        let repo = PluginRepository::default().add_plugin(SmtpProvider);

        let plugin = repo.get("plugin_id").unwrap();

        //let capability = plugin.email_capability().unwrap();

        //let dispatcher = capability.dispatcher(&Configuration::default());
    }
}
