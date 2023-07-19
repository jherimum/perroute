use crate::api::response::{Links, ResourceBuilder, ResourceModel};
use perroute_commons::types::{code::Code, id::Id};
use perroute_storage::models::{message::Message, schema::Version};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

#[derive(Deserialize, Debug, Clone)]
pub struct CreateMessageRequest {
    pub payload: serde_json::Value,
    pub scheduled_to: Option<NaiveDateTime>,
    pub channel_code: Code,
    pub message_type_code: Code,
    pub schema_version: Version,
}

#[derive(Serialize, Debug, Clone)]
pub struct MessageResource {
    id: Id,
}

impl From<&Message> for MessageResource {
    fn from(value: &Message) -> Self {
        Self { id: *value.id() }
    }
}

impl ResourceBuilder<ResourceModel<MessageResource>> for Message {
    fn build(&self, req: &actix_web::HttpRequest) -> ResourceModel<MessageResource> {
        ResourceModel {
            data: Some(self.into()),
            links: Links::default().as_url_map(req),
        }
    }
}
