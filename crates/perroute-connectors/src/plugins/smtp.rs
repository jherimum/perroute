use perroute_commons::types::{recipient::EmailRecipient, Configuration};
use perroute_template::template::{EmailTemplate, RenderedTemplateState};

use crate::{
    generic_plugins::{
        Error, ProviderPlugin, ProviderPluginTrait, Request, Response,
    },
    types::Properties,
};

pub struct SmtpProvider;

impl From<SmtpProvider> for Box<dyn ProviderPluginTrait> {
    fn from(_: SmtpProvider) -> Self {
        Box::new(ProviderPlugin::new("smtp").with_email(
            Properties::default(),
            Box::new(move |config, req| Box::pin(send_email(config, req))),
        ))
    }
}

async fn send_email(
    cfg: &Configuration,
    request: &Request<'_, EmailRecipient, EmailTemplate<RenderedTemplateState>>,
) -> Result<Response, Error> {
    Ok(Response)
}
