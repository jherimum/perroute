use crate::plugin::{
    ConfigurationProperties, DispatchError, DispatchRequest, DispatchResponse, DispatcherPlugin,
};
use lettre::{
    message::{Mailbox, MultiPartBuilder},
    transport::smtp::authentication::Credentials,
    Address, Message, SmtpTransport, Transport,
};
use perroute_commons::types::{dispatch_type::DispatcherType, recipient::Recipient};
use serde::{Deserialize, Serialize};
use std::{str::FromStr, time::Duration};

#[derive(Debug, Deserialize)]
pub struct DispatcherProperties {
    from: Mailbox,
    cc: Vec<Mailbox>,
    bcc: Vec<Mailbox>,
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
            .from_value::<super::connector::Properties>()
            .map_err(DispatchError::unrecoverable)?;

        let credentials = Credentials::new(
            conn_properties.username().to_owned(),
            conn_properties.password().to_owned(),
        );

        let transport = SmtpTransport::relay(&conn_properties.server())
            .unwrap()
            .credentials(credentials)
            .timeout(
                conn_properties
                    .timeout()
                    .map(|to| Duration::from_millis(to)),
            )
            .port(*conn_properties.port())
            .build();

        let response = transport
            .send(&build_message(&req)?)
            .map_err(|e| DispatchError::Unrecoverable(Box::new(e)))?;

        //Ok(DispatchResponse { reference: None })
        todo!()
    }
}

fn build_message(req: &DispatchRequest) -> Result<Message, DispatchError> {
    let disp_properties = req
        .dispatch_properties()
        .from_value::<DispatcherProperties>()
        .map_err(DispatchError::unrecoverable)?;

    let subject = {
        if req.template().is_some() {
            req.template()
                .unwrap()
                .render_subject(&req.into())
                .map_err(|e| DispatchError::Unrecoverable(Box::new(e)))?
        } else {
            None
        }
    };

    let html = {
        if req.template().is_some() {
            req.template()
                .unwrap()
                .render_html(&req.into())
                .map_err(|e| DispatchError::Unrecoverable(Box::new(e)))?
        } else {
            None
        }
    };

    let text = {
        if req.template().is_some() {
            req.template()
                .unwrap()
                .render_text(&req.into())
                .map_err(|e| DispatchError::Unrecoverable(Box::new(e)))?
        } else {
            None
        }
    };

    let multipart = MultiPartBuilder::default().kind(lettre::message::MultiPartKind::Alternative);

    let mut message = Message::builder()
        .to(RecipientMailbox(req.recipient()).try_into()?)
        .from(disp_properties.from)
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

    message
        .multipart(multipart.build())
        .map_err(|e| DispatchError::Unrecoverable(Box::new(e)))
}

pub struct RecipientMailbox<'r>(&'r Recipient);

impl<'r> TryInto<Mailbox> for RecipientMailbox<'r> {
    type Error = DispatchError;

    fn try_into(self) -> Result<Mailbox, Self::Error> {
        Ok(Mailbox {
            name: self.0.name().clone(),
            email: Address::from_str(&self.0.email.as_ref().unwrap())
                .map_err(|e| DispatchError::Unrecoverable(Box::new(e)))?,
        })
    }
}
