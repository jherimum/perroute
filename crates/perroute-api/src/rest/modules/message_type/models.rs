use crate::rest::models::ResourceModel;
use bon::Builder;
use chrono::NaiveDateTime;
use perroute_commons::types::id::Id;
use perroute_storage::models::message_type::MessageType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use validator::Validate;

#[derive(Debug, Deserialize)]
pub struct MessageTypePath(String);

impl MessageTypePath {
    pub fn id(&self) -> Id {
        Id::from(self.0.clone())
    }
}

#[derive(Debug, Deserialize)]
pub struct MessageTypeCollectionPath;

#[derive(Debug, Serialize, Builder)]
pub struct MessageTypeModel {
    pub id: String,
    pub code: String,
    pub name: String,
    pub vars: Option<HashMap<String, String>>,
    pub schema: Value,
    pub enabled: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<&MessageType> for MessageTypeModel {
    fn from(value: &MessageType) -> Self {
        MessageTypeModel::builder()
            .id(value.id.to_string())
            .code(value.code.to_string())
            .name(value.name.to_string())
            .maybe_vars(value.vars.as_ref().map(Into::into))
            .schema((*value.schema).clone())
            .enabled(value.enabled)
            .created_at(*value.created_at)
            .updated_at(*value.updated_at)
            .build()
    }
}

impl From<MessageType> for ResourceModel<MessageType> {
    fn from(value: MessageType) -> Self {
        ResourceModel::new(value.into())
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateMessageTypeRequest {
    pub code: String,
    pub name: String,
    pub vars: Option<HashMap<String, String>>,
    pub schema: Value,
    pub enabled: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateMessageTypeRequest {
    pub name: String,
    pub vars: Option<HashMap<String, String>>,
    pub schema: Value,
    pub enabled: bool,
}
