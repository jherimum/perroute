use crate::api::response::{Links, ResourceBuilder, SingleResourceModel};
use perroute_commons::types::{
    email::Mailbox, id::Id, payload::Payload, phone_number::PhoneNumber,
};
use perroute_connectors::types::delivery::Delivery;
use perroute_storage::models::message::{Message, Status};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, ops::Deref, str::FromStr};
use validator::Validate;

#[derive(Deserialize, Debug, Clone, validator::Validate)]
pub struct CreateMessageRequest {
    pub payload: serde_json::Value,

    #[validate(required)]
    #[validate(custom = "perroute_commons::types::code::Code::validate")]
    pub bu_code: Option<String>,

    #[validate(required)]
    #[validate(custom = "perroute_commons::types::code::Code::validate")]
    pub message_type_code: Option<String>,

    #[validate(required)]
    #[validate(custom = "perroute_commons::types::version::Version::validate")]
    pub schema_version: Option<i32>,

    #[validate]
    #[validate(length(min = 1))]
    pub deliveries: HashSet<DeliveryRest>,
}

#[derive(Serialize, Debug, Clone)]
pub struct MessageResource {
    id: Id,
    status: Status,
    payload: Payload,
    deliveries: HashSet<Delivery>,
}

impl From<&Message> for MessageResource {
    fn from(value: &Message) -> Self {
        Self {
            id: *value.id(),
            payload: value.payload().clone(),
            deliveries: value.deliveries().deref().clone(),
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

impl TryFrom<DeliveryRest> for Delivery {
    type Error = anyhow::Error;
    fn try_from(value: DeliveryRest) -> Result<Self, Self::Error> {
        Ok(match value {
            DeliveryRest::Email(d) => {
                Self::email(Mailbox::from_str(&d.data.unwrap().mailbox.unwrap())?)
            }
            DeliveryRest::Sms(d) => Self::sms(PhoneNumber::from_str(
                &d.data.unwrap().phone_number.unwrap(),
            )?),
            DeliveryRest::Push(_) => Self::push(),
        })
    }
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Hash, Serialize)]
#[serde(untagged)]
pub enum DeliveryRest {
    Email(DeliveryRestData<EmailData>),
    Sms(DeliveryRestData<SmsData>),
    Push(DeliveryRestData<PushData>),
}

impl Validate for DeliveryRest {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            DeliveryRest::Email(d) => d.validate(),
            DeliveryRest::Sms(d) => d.validate(),
            DeliveryRest::Push(d) => d.validate(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Validate)]
pub struct DeliveryRestData<D: Validate + Serialize> {
    #[validate(required)]
    #[validate(custom = "perroute_connectors::types::dispatch_type::DispatchType::validate")]
    dispatch_type: Option<String>,

    #[validate(required)]
    #[validate]
    data: Option<D>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Validate)]
pub struct EmailData {
    #[validate(required)]
    #[validate(custom = "Mailbox::validate")]
    pub mailbox: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Validate)]
pub struct SmsData {
    #[validate(required)]
    #[validate(custom = "perroute_commons::types::phone_number::PhoneNumber::validate")]
    pub phone_number: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Validate)]
pub struct PushData;

pub fn rest_to_delivery(input: HashSet<DeliveryRest>) -> Result<HashSet<Delivery>, anyhow::Error> {
    input.into_iter().map(|d| Delivery::try_from(d)).collect()
}
