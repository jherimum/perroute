use derive_builder::Builder;
use perroute_commons::types::{email::Mailbox, properties::Properties, recipient::Recipient};
use sendgrid::{
    v3::{Email, Message, Personalization, Sender},
    SendgridResult,
};
use serde::Deserialize;
use std::{collections::HashMap, ops::Deref, sync::Arc};

use crate::{
    api::{ConnectorPlugin, DispatchError, DispatchRequest, DispatchResponse, DispatcherPlugin},
    configuration::{Configuration, ConfigurationProperties},
    types::{ConnectorPluginId, DispatchType, TemplateSupport},
};

//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

#[derive(Deserialize, Default)]
pub struct ConnectionProperties {
    api_key: String,
}

impl TryFrom<&Properties> for ConnectionProperties {
    type Error = DispatchError;

    fn try_from(value: &Properties) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Debug)]
pub struct SendGridConnectorPlugin {
    id: ConnectorPluginId,
    configuration: Arc<dyn Configuration>,
    dispatchers: HashMap<DispatchType, Arc<dyn DispatcherPlugin>>,
}

impl ConnectorPlugin for SendGridConnectorPlugin {
    fn id(&self) -> ConnectorPluginId {
        self.id
    }

    fn configuration(&self) -> Arc<dyn Configuration> {
        self.configuration.clone()
    }

    fn dispatchers(&self) -> &HashMap<DispatchType, Arc<dyn DispatcherPlugin>> {
        &self.dispatchers
    }
}

#[derive(Deserialize, Builder)]
pub struct EmailDispatcherProperties {
    from: Mailbox,
    template_id: Option<String>,
    categories: Vec<String>,
}

#[derive(Debug)]
pub struct EmailDispatcherPlugin<'c> {
    connector_plugin: &'c SendGridConnectorPlugin,
    template_support: TemplateSupport,
    dispatch_type: DispatchType,
    configuration: ConfigurationProperties,
}

impl<'c> EmailDispatcherPlugin<'c> {
    fn new(connector_plugin: &'c SendGridConnectorPlugin) -> Self {
        Self {
            connector_plugin: connector_plugin,
            template_support: TemplateSupport::Optional,
            dispatch_type: DispatchType::Email,
            configuration: Default::default(),
        }
    }
}

#[async_trait::async_trait]
impl<'c> DispatcherPlugin for EmailDispatcherPlugin<'c> {
    fn template_support(&self) -> TemplateSupport {
        self.template_support
    }

    fn dispatch_type(&self) -> DispatchType {
        self.dispatch_type
    }

    fn configuration(&self) -> Arc<dyn Configuration> {
        //&self.configuration
        todo!()
    }

    async fn dispatch(&self, req: &DispatchRequest) -> Result<DispatchResponse, DispatchError> {
        // let conn_properties = self
        //     .connector_plugin
        //     .configuration
        //     .build::<ConnectionProperties>(req.connection_properties)
        //     .unwrap();
        let conn_properties = ConnectionProperties::default();

        let sender = Sender::new(conn_properties.api_key);
        let message = build_message(req).unwrap();
        println!("{}", serde_json::to_string_pretty(&message).unwrap());
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

        Ok(DispatchResponse::new(None, None))
    }
}

fn build_message(req: &DispatchRequest) -> SendgridResult<Message> {
    // let disp_props = req
    //     .dispatch_properties()
    //     .from_value::<EmailDispatcherProperties>()
    //     .unwrap();

    let disp_props = EmailDispatcherPropertiesBuilder::default().build().unwrap();

    let mut message = Message::new(SendGridEmail::from(disp_props.from).into());
    if disp_props.template_id.is_some() {
        message = message.set_template_id(disp_props.template_id.as_ref().unwrap());
    }

    Ok(message
        .add_personalization(personalization_from_request(&req)?)
        .add_categories(&disp_props.categories)
        .set_subject(&req.subject().as_ref().cloned().unwrap_or_default()))
}

fn personalization_from_request(req: &DispatchRequest) -> SendgridResult<Personalization> {
    let email: SendGridEmail = req.recipient().into();
    Personalization::new(email.into()).add_dynamic_template_data_json(req.payload())
}

pub struct SendGridEmail(Email);

impl Into<Email> for SendGridEmail {
    fn into(self) -> Email {
        self.0
    }
}

impl Deref for SendGridEmail {
    type Target = Email;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Mailbox> for SendGridEmail {
    fn from(mail_box: Mailbox) -> Self {
        let mut email = Email::new(mail_box.deref().email.to_string());

        if mail_box.deref().name.is_some() {
            email = email.set_name(mail_box.deref().name.as_ref().unwrap().to_string());
        }

        SendGridEmail(email)
    }
}

impl From<&Recipient> for SendGridEmail {
    fn from(value: &Recipient) -> Self {
        let email: Mailbox = value.into();
        email.into()
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
