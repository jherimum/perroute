use crate::{DispatchError, DispatchResponse, Dispatcher, DispatcherTrait, ProviderPlugin};
use perroute_commons::types::{recipient::EmailRecipient, template::EmailTemplate, Configuration};

pub mod smtp;

pub struct SmtpPlugin;

impl ProviderPlugin for SmtpPlugin {
    fn email_dispatcher(
        &self,
        configuration: Configuration,
    ) -> Option<Box<dyn DispatcherTrait<EmailRecipient, EmailTemplate>>> {
        Some(Box::new(Dispatcher::new(configuration, function)))
    }
}

async fn function(
    configuration: Configuration,
    recipient: EmailRecipient,
    template: EmailTemplate,
) -> Result<DispatchResponse, DispatchError> {
    todo!()
}
