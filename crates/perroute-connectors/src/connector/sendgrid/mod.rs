use crate::{
    api::{
        BaseConnectorPlugin, BaseDispatcherPlugin, ConnectorPlugin, DispatchError, DispatchRequest,
        DispatchResponse,
    },
    configuration::{ConfigurationProperties, DefaultConfiguration},
    types::{dispatch_type::DispatchType, plugin_id::ConnectorPluginId},
};
use perroute_commons::types::email::Mailbox;
use sendgrid::{
    v3::{Email, Message, Personalization, Sender},
    SendgridResult,
};
use serde::Deserialize;
use std::{marker::PhantomData, ops::Deref};
use validator::Validate;

pub fn sendgrid_connector_plugin() -> impl ConnectorPlugin {
    BaseConnectorPlugin::new(
        ConnectorPluginId::Sendgrid,
        Box::new(DefaultConfiguration::new(
            connection_properties(),
            PhantomData::<SendgridConnectionProperties>,
        )),
        vec![Box::new(BaseDispatcherPlugin::new(
            DispatchType::Email,
            Box::new(DefaultConfiguration::new(
                dispatcher_properties(),
                PhantomData::<EmailDispatcherProperties>,
            )),
            |req| Box::pin(dispatch(req)),
        ))],
    )
}

#[derive(Deserialize, Default, Debug)]
pub struct SendgridConnectionProperties {
    api_key: String,
}

impl Validate for SendgridConnectionProperties {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        Ok(())
    }
}

fn connection_properties() -> ConfigurationProperties {
    ConfigurationProperties::default()
}

#[derive(Debug, Deserialize)]
pub struct EmailDispatcherProperties {
    from: Mailbox,
    categories: Vec<String>,
}

impl Validate for EmailDispatcherProperties {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        Ok(())
    }
}

fn dispatcher_properties() -> ConfigurationProperties {
    ConfigurationProperties::default()
}

pub async fn dispatch<'r>(req: &DispatchRequest<'r>) -> Result<DispatchResponse, DispatchError> {
    let conn_properties = req
        .connection_properties
        .from_value::<SendgridConnectionProperties>()
        .unwrap();

    let sender = Sender::new(conn_properties.api_key);
    let message = build_message(req).unwrap();
    let r = sender.send(&message).await;

    if r.is_err() {
        let x = r.err();
    } else {
        let x = r.ok();
        if x.is_some() {
            let x = x.unwrap();
            dbg!(&x.status());
            dbg!(x.text().await);
        } else {
            println!("nadaaa");
        }
    }

    Ok(DispatchResponse {
        reference: None,
        data: None,
    })
}

//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

fn build_message(req: &DispatchRequest) -> SendgridResult<Message> {
    let disp_properties = req
        .dispatch_properties
        .from_value::<EmailDispatcherProperties>()
        .unwrap();

    let message = Message::new(SendGridEmail::from(&disp_properties.from).into());

    Ok(
        message
            .add_personalization(personalization_from_request(req))
            .add_categories(&disp_properties.categories), //    .set_subject(&req.subject().as_ref().cloned().unwrap_or_default())
    )
}

fn personalization_from_request(req: &DispatchRequest) -> Personalization {
    let email: SendGridEmail = req.delivery().email_data().unwrap().mailbox().into();
    Personalization::new(email.into())
}

pub struct SendGridEmail(Email);

impl From<SendGridEmail> for Email {
    fn from(val: SendGridEmail) -> Self {
        val.0
    }
}

impl Deref for SendGridEmail {
    type Target = Email;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&Mailbox> for SendGridEmail {
    fn from(mail_box: &Mailbox) -> Self {
        let mut email = Email::new(mail_box.deref().email.to_string());

        if mail_box.deref().name.is_some() {
            email = email.set_name(mail_box.deref().name.as_ref().unwrap().to_string());
        }

        Self(email)
    }
}

// #[cfg(test)]
// mod tests {
//     use std::str::FromStr;

//     use perroute_commons::{
//         new_id,
//         types::{
//             email::Email, payload::Payload, properties::Properties, recipient::Recipient,
//             vars::Vars,
//         },
//     };
//     use serde_json::json;

//     use crate::api::{DispatchRequest, DispatcherPlugin};

//     use super::EmailDispatcherPlugin;

//     #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
//     async fn name() {
//         let email = Email::from_str("eugenio@wine.com.br").unwrap();

//         let dis = EmailDispatcherPlugin::default();
//         let req = DispatchRequest {
//             id: new_id!(),
//             connection_properties: &Properties::new(
//                 json!({"api_key": "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"}),
//             ),
//             dispatch_properties: &Properties::new(json!({
//                 "from": {
//                     "name": "Eugenio",
//                     "email": "eugenio.perrottaneto@gmail.com"
//                 },
//                 "categories": [],
//                 "template_id": "d-f6452b517f9d424082f38fae89d1b650"
//             })),
//             template: None,
//             recipient: &Recipient {
//                 name: Some("eugenio".to_owned()),
//                 email: Some(email),
//                 phone_number: None,
//             },
//             payload: &Payload::new(json!({"nome": "Eugenio"})),
//             vars: &Vars::default(),
//             subject: Some("ola".to_owned()),
//         };

//         let r = dis.dispatch(&req).await;
//         dbg!(r);
//     }
// }
