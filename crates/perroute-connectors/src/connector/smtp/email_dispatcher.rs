use super::connector::SmtpConnectorProperties;
use crate::plugin::{
    ConfigurationProperties, DispatchError, DispatchRequest, DispatchResponse, DispatchTemplate,
    DispatcherPlugin,
};
use derive_builder::Builder;
use lettre::{
    message::{Mailbox, MaybeString, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    Address, Message, SmtpTransport, Transport,
};
use perroute_commons::types::{
    dispatch_type::DispatcherType,
    recipient::Recipient,
    template::{TemplateData, TemplateError},
};
use serde::{Deserialize, Serialize};
use std::{ops::Deref, str::FromStr, time::Duration};

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

#[derive(Debug, Default)]
pub struct EmailDispatcher {
    configuration: ConfigurationProperties,
}

#[derive(Serialize)]
pub struct Response {}

impl DispatcherPlugin for EmailDispatcher {
    fn dispatch_type(&self) -> DispatcherType {
        DispatcherType::Email
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

        let response = transport
            .send(&Message::try_from(req)?)
            .map_err(|e| DispatchError::Unrecoverable(Box::new(e)))?;

        Ok(DispatchResponse {
            reference: None,
            data: Some(serde_json::to_value(response).unwrap()),
        })
    }
}

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
            .map_err(|e| DispatchError::Unrecoverable(Box::new(e)))?;

        let html = req
            .template()
            .and_then(|e| Some(e.render_html(&req.into())))
            .unwrap_or(Ok(None))
            .map_err(|e| DispatchError::Unrecoverable(Box::new(e)))?
            .map(|html| SinglePart::html(html));

        let text = req
            .template()
            .and_then(|e| Some(e.render_text(&req.into())))
            .unwrap_or(Ok(None))
            .map_err(|e| DispatchError::Unrecoverable(Box::new(e)))?
            .map(|html| SinglePart::html(html));

        let mut message = Message::builder()
            .to(RecipientMailbox(req.recipient()).try_into()?)
            .from(disp_properties.from)
            .date_now()
            .subject(subject.unwrap_or(String::default()));

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
            (None, None) => message
                .singlepart(SinglePart::builder().body(MaybeString::String(String::default()))),
            (None, Some(plain)) => message.singlepart(plain),
            (Some(html), None) => message.singlepart(html),
            (Some(html), Some(plain)) => {
                message.multipart(MultiPart::alternative().singlepart(html).singlepart(plain))
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
        let email = self
            .email()
            .as_ref()
            .map(|email| Address::from_str(&email))
            .transpose()
            .map_err(|e| DispatchError::Unrecoverable(Box::new(e)))?
            .unwrap(); //TODO: error hahndler

        Ok(Mailbox {
            name: self.name().clone(),
            email,
        })
    }
}

impl TryFrom<&SmtpConnectorProperties> for SmtpTransport {
    type Error = DispatchError;
    fn try_from(value: &SmtpConnectorProperties) -> Result<Self, Self::Error> {
        let credentials =
            Credentials::new(value.username().to_owned(), value.password().to_owned());
        Ok(SmtpTransport::starttls_relay(&value.host())
            .map_err(DispatchError::unrecoverable)?
            .credentials(credentials)
            .timeout(value.timeout().map(|to| Duration::from_millis(to)))
            .port(*value.port())
            .build())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{
        connector::smtp::connector::SmtpConnectorPropertiesBuilder,
        plugin::{DispatchRequest, DispatcherPlugin},
    };
    use lettre::message::Mailbox;
    use perroute_commons::types::{id::Id, payload::Payload, properties::Properties, vars::Vars};
    use std::str::FromStr;

    #[test]
    fn name() {
        let conn_props = SmtpConnectorPropertiesBuilder::default()
            .port(587)
            .host("smtp-relay.brevo.com".to_owned())
            .username("eugenio.perrottaneto@gmail.com".to_owned())
            .password("xsmtpsib-3361a853fc44e605522f628393be7d82a0074fba4e7aa9f93c53bbadfbfaa41e-VO8M5nd2f6jGsLU1".to_owned())
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
                email: Some("eugenio.perrottaneto@gmail.com".to_owned()),
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
    fn render_subject(&self, data: &TemplateData) -> Result<Option<String>, TemplateError> {
        Ok(Some("assunto".to_owned()))
    }
    fn render_text(&self, data: &TemplateData) -> Result<Option<String>, TemplateError> {
        Ok(None)
    }
    fn render_html(&self, data: &TemplateData) -> Result<Option<String>, TemplateError> {
        Ok(Some("<h1>Eugenio</h1>".to_owned()))
    }
}
