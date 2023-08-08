use super::connector::SmtpConnectorProperties;
use crate::api::{
    ConfigurationProperties, DispatchError, DispatchRequest, DispatchResponse, DispatchTemplate,
    DispatchType, DispatcherPlugin, ResponseData, TemplateSupport,
};
use derive_builder::Builder;
use lettre::{
    message::{Mailbox, MaybeString, MultiPart, SinglePart},
    transport::smtp::{authentication::Credentials, response::Response},
    Message, SmtpTransport, Transport,
};
use perroute_commons::types::{
    recipient::Recipient,
    template::{TemplateData, TemplateError},
};
use serde::{Deserialize, Serialize};
use std::{ops::Deref, time::Duration};
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum EmailDispatcherError {
    #[error("email not supplied")]
    EmailNotSupplied,
}

impl From<EmailDispatcherError> for DispatchError {
    fn from(e: EmailDispatcherError) -> Self {
        Self::Unrecoverable(Box::new(e))
    }
}

#[derive(Debug, Deserialize, Builder, Serialize)]
pub struct EmailDispatcherProperties {
    from: Mailbox,

    #[builder(default)]
    cc: Vec<Mailbox>,
    #[builder(default)]
    bcc: Vec<Mailbox>,
    #[builder(default)]
    reply_to: Vec<Mailbox>,
}

#[derive(Debug)]
pub struct EmailDispatcher {
    configuration: ConfigurationProperties,
    dispatch_type: DispatchType,
    template_support: TemplateSupport,
}

impl Default for EmailDispatcher {
    fn default() -> Self {
        Self {
            configuration: ConfigurationProperties::default(),
            dispatch_type: DispatchType::Email,
            template_support: TemplateSupport::Mandatory,
        }
    }
}

impl DispatcherPlugin for EmailDispatcher {
    fn template_support(&self) -> TemplateSupport {
        self.template_support
    }

    fn dispatch_type(&self) -> DispatchType {
        self.dispatch_type
    }

    fn configuration(&self) -> &ConfigurationProperties {
        &self.configuration
    }

    fn dispatch(&self, req: &DispatchRequest) -> Result<DispatchResponse, DispatchError> {
        let conn_properties = req
            .connection_properties()
            .from_value::<SmtpConnectorProperties>()
            .map_err(DispatchError::unrecoverable)?;

        let transport = SmtpTransport::try_from(&conn_properties)?;

        transport
            .send(&Message::try_from(req)?)
            .map_err(|e| DispatchError::Unrecoverable(Box::new(e)))
            .map(|response| DispatchResponse::new(None, Some(Box::new(EmailResponse(response)))))
    }
}

#[derive(Serialize, Debug)]
pub struct EmailResponse(Response);
impl ResponseData for EmailResponse {}

impl TryFrom<&DispatchRequest<'_, '_, '_, '_, '_, '_>> for Message {
    type Error = DispatchError;

    fn try_from(req: &DispatchRequest) -> Result<Self, Self::Error> {
        let disp_properties = req
            .dispatch_properties()
            .from_value::<EmailDispatcherProperties>()
            .map_err(DispatchError::unrecoverable)?;

        let subject = req
            .template()
            .and_then(|e| Some(e.render_subject(&req.into())))
            .unwrap_or(Ok(None))
            .map_err(DispatchError::from)?;

        let html = req
            .template()
            .and_then(|e| Some(e.render_html(&req.into())))
            .unwrap_or(Ok(None))
            .map_err(DispatchError::from)?;

        let text = req
            .template()
            .and_then(|e| Some(e.render_text(&req.into())))
            .unwrap_or(Ok(None))
            .map_err(DispatchError::from)?;

        let mut message = Self::builder()
            .to(RecipientMailbox(req.recipient()).try_into()?)
            .from(disp_properties.from)
            .date_now()
            .subject(subject.unwrap_or_default());

        for m in disp_properties.bcc {
            message = message.bcc(m.clone());
        }

        for m in disp_properties.cc {
            message = message.cc(m.clone());
        }

        for m in disp_properties.reply_to {
            message = message.reply_to(m.clone());
        }

        match (html, text) {
            (None, None) => {
                message.singlepart(SinglePart::plain(MaybeString::String(String::default())))
            }
            (None, Some(plain)) => message.singlepart(SinglePart::plain(plain)),
            (Some(html), None) => message.singlepart(SinglePart::html(html)),
            (Some(html), Some(plain)) => {
                message.multipart(MultiPart::alternative_plain_html(plain, html))
            }
        }
        .map_err(DispatchError::unrecoverable)
    }
}

pub struct RecipientMailbox<'r>(&'r Recipient);

impl Deref for RecipientMailbox<'_> {
    type Target = Recipient;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'r> TryInto<Mailbox> for RecipientMailbox<'r> {
    type Error = DispatchError;

    fn try_into(self) -> Result<Mailbox, Self::Error> {
        Ok(self
            .email()
            .as_ref()
            .map(|addr| Mailbox::new(self.name().clone(), addr.deref().clone()))
            .ok_or(EmailDispatcherError::EmailNotSupplied)?)
    }
}

impl TryFrom<&SmtpConnectorProperties> for SmtpTransport {
    type Error = DispatchError;
    fn try_from(value: &SmtpConnectorProperties) -> Result<Self, Self::Error> {
        Ok(if *value.starttls() {
            SmtpTransport::starttls_relay(value.host())
        } else {
            SmtpTransport::relay(value.host())
        }
        .map_err(DispatchError::unrecoverable)
        .tap_err(|e| tracing::error!("Failed to build SmtpTransport: {e}"))?
        .credentials(Credentials::new(
            value.username().to_owned(),
            value.password().to_owned(),
        ))
        .timeout(value.timeout().map(Duration::from_millis))
        .port(*value.port())
        .build())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{
        api::{DispatchRequest, DispatcherPlugin},
        connector::smtp::connector::SmtpConnectorPropertiesBuilder,
    };
    use lettre::message::Mailbox;
    use perroute_commons::types::{
        id::Id, payload::Payload, properties::Properties, recipient::Email, vars::Vars,
    };
    use std::str::FromStr;

    #[test]
    fn name() {
        let conn_props = SmtpConnectorPropertiesBuilder::default()
            .port(587)
            .host("smtp-relay.brevo.com".to_owned())
            .username("eugenio.perrottaneto@gmail.com".to_owned())
            .password("xsmtpsib-3361a853fc44e605522f628393be7d82a0074fba4e7aa9f93c53bbadfbfaa41e-GQgfZYFv54N2BUS3".to_owned())
            .timeout(Some(1000))
            .build().unwrap();

        let disp_props = EmailDispatcherPropertiesBuilder::default()
            .from(Mailbox::from_str("eugenio.perrottaneto@gmail.com").unwrap())
            .build()
            .unwrap();

        let req: DispatchRequest = DispatchRequest {
            id: Id::new(),
            connection_properties: &Properties::new(serde_json::to_value(conn_props).unwrap()),
            dispatch_properties: &Properties::new(serde_json::to_value(disp_props).unwrap()),
            template: Some(&Temp as &dyn DispatchTemplate),
            recipient: &perroute_commons::types::recipient::Recipient {
                name: Some("eugenio".to_owned()),
                email: Some(Email::from_str("eugenio.perrottaneto@gmail.com").unwrap()),
                phone_number: None,
            },
            payload: &Payload::default(),
            vars: &Vars::default(),
        };
        let res = EmailDispatcher::default().dispatch(&req);
        dbg!(&res);
    }
}

pub struct Temp;

impl DispatchTemplate for Temp {
    fn render_subject(&self, _: &TemplateData) -> Result<Option<String>, TemplateError> {
        Ok(Some("assunto".to_owned()))
    }
    fn render_text(&self, _: &TemplateData) -> Result<Option<String>, TemplateError> {
        //Ok(Some("TEXT".to_owned()))
        Ok(None)
    }
    fn render_html(&self, _: &TemplateData) -> Result<Option<String>, TemplateError> {
        //Ok(Some("<h1>Eugenio</h1>".to_owned()))
        Ok(None)
    }
}
