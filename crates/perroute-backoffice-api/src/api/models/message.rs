use crate::api::response::{Links, ResourceBuilder, SingleResourceModel};
use perroute_commons::types::{code::Code, id::Id, payload::Payload, recipient::Recipient};
use perroute_connectors::DispatcherType;
use perroute_storage::models::{
    message::{Message, Status},
    schema::Version,
};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use std::collections::HashSet;

#[derive(Deserialize, Debug, Clone)]
pub struct CreateMessageRequest {
    pub payload: serde_json::Value,
    #[serde(default)]
    pub scheduled_to: Option<NaiveDateTime>,
    pub channel_code: Code,
    pub message_type_code: Code,
    pub schema_version: Version,
    #[serde(default)]
    pub include_dispatcher_types: HashSet<DispatcherType>,
    #[serde(default)]
    pub exclude_dispatcher_types: HashSet<DispatcherType>,

    pub recipient: Recipient,
}

#[derive(Serialize, Debug, Clone)]
pub struct MessageResource {
    id: Id,
    recipient: Recipient,
    status: Status,
    payload: Payload,
    pub include_dispatcher_types: HashSet<DispatcherType>,
    pub exclude_dispatcher_types: HashSet<DispatcherType>,
}

impl From<&Message> for MessageResource {
    fn from(value: &Message) -> Self {
        Self {
            id: *value.id(),
            payload: value.payload().clone(),
            recipient: value.recipient().as_ref().clone(),
            include_dispatcher_types: value.include_dispatcher_types().as_ref().clone(),
            exclude_dispatcher_types: value.exclude_dispatcher_types().as_ref().clone(),
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
