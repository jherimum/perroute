use perroute_commons::types::{code::Code, id::Id};
use perroute_storage::models::message_type::MessageType;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CreateMessageTypeRequest {
    pub code: Code,
    pub description: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateMessageTypeRequest {
    pub description: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct MessageTypeResource {
    pub id: Id,
    pub code: Code,
    pub description: String,
    pub enabled: bool,
    pub channel_id: Id,
}

impl From<MessageType> for MessageTypeResource {
    fn from(value: MessageType) -> Self {
        Self {
            id: *value.id(),
            code: value.code().clone(),
            description: value.description().clone(),
            enabled: *value.enabled(),
            channel_id: *value.channel_id(),
        }
    }
}
