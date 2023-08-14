use crate::api::response::{Links, ResourceBuilder, SingleResourceModel};
use perroute_commons::types::{code::Code, id::Id, payload::Payload, recipient::Recipient};
use perroute_connectors::api::DispatchType;
use perroute_storage::models::{
    message::{Message, Status},
    schema::Version,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Deserialize, Debug, Clone)]
pub struct CreateMessageRequest {
    pub payload: serde_json::Value,
    pub bu_code: Code,
    pub message_type_code: Code,
    pub schema_version: Version,
    pub dispatcher_types: HashSet<DispatchType>,
    pub recipient: Recipient,
}

#[derive(Serialize, Debug, Clone)]
pub struct MessageResource {
    id: Id,
    recipient: Recipient,
    status: Status,
    payload: Payload,
    pub dispatcher_types: HashSet<DispatchType>,
}

impl From<&Message> for MessageResource {
    fn from(value: &Message) -> Self {
        Self {
            id: *value.id(),
            payload: value.payload().clone(),
            recipient: value.recipient().as_ref().clone(),
            dispatcher_types: value.dispatcher_types().as_ref().clone(),
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
