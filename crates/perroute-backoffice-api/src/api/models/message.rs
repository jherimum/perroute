use std::{collections::HashSet, str::FromStr};

use crate::api::response::{Links, ResourceBuilder, SingleResourceModel};
use perroute_commons::types::{
    code::Code, email::Mailbox, id::Id, payload::Payload, phonenumber::PhoneNumber,
    recipient::Recipient, version::Version,
};
use perroute_connectors::types::delivery::Delivery;
use perroute_storage::models::message::{Deliveries, Message, Status};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct CreateMessageRequest {
    pub payload: serde_json::Value,
    pub bu_code: Code,
    pub message_type_code: Code,
    pub schema_version: Version,
    pub deliveries: HashSet<DeliveryRest>,
    pub recipient: Recipient,
}

#[derive(Serialize, Debug, Clone)]
pub struct MessageResource {
    id: Id,
    recipient: Recipient,
    status: Status,
    payload: Payload,
    deliveries: HashSet<DeliveryRest>,
}

impl From<&Message> for MessageResource {
    fn from(value: &Message) -> Self {
        Self {
            id: *value.id(),
            payload: value.payload().clone(),
            recipient: value.recipient().clone(),
            deliveries: value.deliveries().iter().map(Into::into).collect(),
            status: *value.status(),
        }
    }
}

impl ResourceBuilder<SingleResourceModel<MessageResource>> for Message {
    fn build(&self, req: &actix_web::HttpRequest) -> SingleResourceModel<MessageResource> {
        SingleResourceModel {
            data: Some(self.into()),
            links: Links::default().as_url_map(req),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Hash, Serialize)]
pub enum DeliveryRest {
    Email { mailbox: String },
    Sms { phone_number: String },
    Push,
}

impl From<&Delivery> for DeliveryRest {
    fn from(value: &Delivery) -> Self {
        // match value {
        //     Delivery::Email(data) => DeliveryRest::Email {
        //         mailbox: data.data.mailbox.to_string(),
        //     },
        //     Delivery::Sms(data) => DeliveryRest::Sms {
        //         phone_number: data.data.phone_number.to_string(),
        //     },
        //     Delivery::Push(_) => DeliveryRest::Push,
        // }
        todo!()
    }
}

impl TryInto<Delivery> for DeliveryRest {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Delivery, Self::Error> {
        todo!()
    }
}
