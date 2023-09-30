use crate::api::response::{Links, ResourceBuilder, SingleResourceModel};
use anyhow::{Context, Result};
use perroute_commons::types::{
    code::Code, email::Mailbox, id::Id, payload::Payload, phone_number::PhoneNumber,
    version::Version,
};
use perroute_connectors::types::recipient::Recipient;
use perroute_storage::models::message::{Message, Status};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, ops::Deref, str::FromStr};
use validator::Validate;

#[derive(Deserialize, Debug, Clone, validator::Validate)]
pub struct CreateMessageRequest {
    #[validate(required)]
    payload: Option<serde_json::Value>,

    #[validate(required)]
    #[validate(custom = "Code::validate")]
    business_unit_code: Option<String>,

    #[validate(required)]
    #[validate(custom = "Code::validate")]
    message_type_code: Option<String>,

    #[validate]
    #[validate(length(min = 1))]
    recipients: HashSet<RecipientRest>,
}

impl CreateMessageRequest {
    pub fn payload(&self) -> Result<Payload> {
        Ok(self
            .payload
            .clone()
            .context("missing payload")?
            .clone()
            .into())
    }

    pub fn business_unit_code(&self) -> Result<Code> {
        self.business_unit_code
            .clone()
            .context("missing business unit code")?
            .try_into()
            .context("invalid business unit code")
    }

    pub fn message_type_code(&self) -> Result<Code> {
        self.message_type_code
            .clone()
            .context("missing message type code")?
            .try_into()
            .context("invalid message type code ")
    }

    pub fn recipients(&self) -> Result<HashSet<Recipient>> {
        self.recipients
            .clone()
            .into_iter()
            .map(Recipient::try_from)
            .collect()
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct MessageResource {
    id: Id,
    status: Status,
    payload: Payload,
    recipients: HashSet<Recipient>,
}

impl From<&Message> for MessageResource {
    fn from(value: &Message) -> Self {
        Self {
            id: *value.id(),
            payload: value.payload().clone(),
            recipients: value.recipients().deref().clone(),
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

impl TryFrom<RecipientRest> for Recipient {
    type Error = anyhow::Error;
    fn try_from(value: RecipientRest) -> Result<Self, Self::Error> {
        Ok(match value {
            RecipientRest::Email(d) => {
                Self::email(Mailbox::from_str(&d.data.unwrap().mailbox.unwrap())?)
            }
            RecipientRest::Sms(d) => Self::sms(PhoneNumber::from_str(
                &d.data.unwrap().phone_number.unwrap(),
            )?),
            RecipientRest::Push(_) => Self::push(),
        })
    }
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Hash, Serialize)]
#[serde(untagged)]
pub enum RecipientRest {
    Email(RecipientRestData<EmailData>),
    Sms(RecipientRestData<SmsData>),
    Push(RecipientRestData<PushData>),
}

impl Validate for RecipientRest {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            RecipientRest::Email(d) => d.validate(),
            RecipientRest::Sms(d) => d.validate(),
            RecipientRest::Push(d) => d.validate(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Validate)]
pub struct RecipientRestData<D: Validate + Serialize> {
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
