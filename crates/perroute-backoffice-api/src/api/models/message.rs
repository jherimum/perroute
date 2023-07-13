use perroute_commons::types::code::Code;
use perroute_storage::models::{message::Message, schema::Version};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

use crate::api::response::{ResourceBuilder, ResourceModel};

#[derive(Deserialize, Debug, Clone)]
pub struct CreateMessageRequest {
    pub payload: serde_json::Value,
    pub scheduled_to: Option<NaiveDateTime>,
    message_type_code: Code,
    schema_version: Version,
}

#[derive(Serialize, Debug, Clone)]
pub struct MessageResource {}

impl From<Message> for MessageResource {
    fn from(value: Message) -> Self {
        todo!()
    }
}

impl ResourceBuilder<ResourceModel<MessageResource>> for Message {
    fn build(&self, req: &actix_web::HttpRequest) -> ResourceModel<MessageResource> {
        todo!()
    }
}
