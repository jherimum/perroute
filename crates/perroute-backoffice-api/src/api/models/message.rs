use crate::api::response::{Links, ResourceBuilder, SingleResourceModel};
use perroute_commons::types::{
    code::Code, id::Id, payload::Payload, recipient::Recipient, version::Version,
};
use perroute_connectors::types::DispatchTypes;
use perroute_storage::models::message::{Message, Status};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct CreateMessageRequest {
    pub payload: serde_json::Value,
    pub bu_code: Code,
    pub message_type_code: Code,
    pub schema_version: Version,
    pub dispatcher_types: DispatchTypes,
    pub recipient: Recipient,
}

#[derive(Serialize, Debug, Clone)]
pub struct MessageResource {
    id: Id,
    recipient: Recipient,
    status: Status,
    payload: Payload,
    pub dispatcher_types: DispatchTypes,
}

impl From<&Message> for MessageResource {
    fn from(value: &Message) -> Self {
        Self {
            id: *value.id(),
            payload: value.payload().clone(),
            recipient: value.recipient().clone(),
            dispatcher_types: value.dispatcher_types().clone(),
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
