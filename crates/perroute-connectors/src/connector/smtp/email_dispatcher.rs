use super::SmtpConnectorProperties;
use crate::{
    api::{
        BaseDispatcherPlugin, DispatchError, DispatchRequest, DispatchResponse, DispatcherPlugin,
        ResponseData,
    },
    configuration::{
        ConfigurationProperties, ConfigurationPropertyBuilder, ConfigurationPropertyType,
        DefaultConfiguration,
    },
    types::{DispatchType, TemplateSupport},
};
use derive_builder::Builder;
use lettre::{
    message::{MaybeString, MultiPart, SinglePart},
    transport::smtp::{authentication::Credentials, response::Response},
    Message, SmtpTransport, Transport,
};
use perroute_commons::types::{email::Mailbox, recipient::Recipient};
use serde::{Deserialize, Serialize};
use std::{ops::Deref, sync::Arc, time::Duration};
use tap::TapFallible;

pub fn email_dispatcher() -> impl DispatcherPlugin {
    BaseDispatcherPlugin::new(
        DispatchType::Email,
        TemplateSupport::Mandatory,
        Arc::new(DefaultConfiguration::default()),
        |req| Box::pin(dispatch(req)),
    )
}

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

pub async fn dispatch<'r>(req: &DispatchRequest<'r>) -> Result<DispatchResponse, DispatchError> {
    let conn_properties = req
        .connection_properties
        .from_value::<SmtpConnectorProperties>()
        .unwrap();

    let transport = SmtpTransport::try_from(&conn_properties)?;

    transport
        .send(&Message::try_from(req)?)
        .map_err(|e| DispatchError::Unrecoverable(Box::new(e)))
        .map(|response| DispatchResponse::new(None, Some(Box::new(EmailResponse(response)))))
}

pub fn properties() -> ConfigurationProperties {
    [
        ConfigurationPropertyBuilder::default()
            .name("from")
            .required(true)
            .description("from Mailbox")
            .property_type(ConfigurationPropertyType::String)
            .multiple(false)
            .build()
            .unwrap(),
        ConfigurationPropertyBuilder::default()
            .name("cc")
            .required(false)
            .description("cc Mailbox list")
            .property_type(ConfigurationPropertyType::String)
            .multiple(true)
            .build()
            .unwrap(),
        ConfigurationPropertyBuilder::default()
            .name("bcc")
            .required(false)
            .description("bcc Mailbox list")
            .property_type(ConfigurationPropertyType::String)
            .multiple(true)
            .build()
            .unwrap(),
        ConfigurationPropertyBuilder::default()
            .name("reply_to")
            .required(false)
            .description("reply_to Mailbox list")
            .property_type(ConfigurationPropertyType::String)
            .multiple(true)
            .build()
            .unwrap(),
    ]
    .into()
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

#[derive(Serialize, Debug)]
pub struct EmailResponse(pub Response);
impl ResponseData for EmailResponse {}

impl TryFrom<&DispatchRequest<'_>> for Message {
    type Error = DispatchError;

    fn try_from(req: &DispatchRequest) -> Result<Self, Self::Error> {
        // let disp_properties = req
        //     .dispatch_properties()
        //     .from_value::<EmailDispatcherProperties>()
        //     .map_err(DispatchError::unrecoverable)?;

        let disp_properties = EmailDispatcherPropertiesBuilder::default().build().unwrap();

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

        let recipient_mail_box: Mailbox = req.recipient().into();

        let mut message = Self::builder()
            .to(recipient_mail_box.into())
            .from(disp_properties.from.deref().clone())
            .date_now()
            .subject(req.subject().clone().unwrap_or_default());

        for m in disp_properties.bcc {
            message = message.bcc(m.into());
        }

        for m in disp_properties.cc {
            message = message.cc(m.into());
        }

        for m in disp_properties.reply_to {
            message = message.reply_to(m.into());
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

/*
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

    #[tokio::test]
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
        let res = EmailDispatcher::default().dispatch(&req).await;
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
 */
