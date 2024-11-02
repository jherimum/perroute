use crate::rest::{error::ApiError, models::ResourceModel};
use bon::Builder;
use chrono::NaiveDateTime;
use perroute_commons::types::{id::Id, name::Name, Payload};
use perroute_storage::models::message_type::{MessageType, PayloadExample};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, ops::Deref};
use validator::Validate;

#[derive(Debug, Deserialize)]
pub struct MessageTypePath(String);

impl MessageTypePath {
    pub fn id(&self) -> Id {
        Id::from(self.0.clone())
    }

    pub fn parent(self) -> MessageTypeCollectionPath {
        MessageTypeCollectionPath
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
    pub payload_examples: Vec<(String, Value)>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<&(MessageType, Vec<PayloadExample>)> for MessageTypeModel {
    fn from(value: &(MessageType, Vec<PayloadExample>)) -> Self {
        MessageTypeModel::builder()
            .id(value.0.id().to_string())
            .code(value.0.code().to_string())
            .name(value.0.name().to_string())
            .maybe_vars(value.0.vars().as_ref().map(Into::into))
            .schema(value.0.schema().deref().clone())
            .enabled(*value.0.enabled())
            .created_at(**value.0.created_at())
            .updated_at(**value.0.updated_at())
            .payload_examples(value.1.iter().map(Into::into).collect())
            .build()
    }
}

impl From<&(MessageType, Vec<PayloadExample>)> for ResourceModel<MessageTypeModel> {
    fn from(value: &(MessageType, Vec<PayloadExample>)) -> Self {
        ResourceModel::new(value.into())
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct PayloadExampleModel {
    name: String,
    payload: Value,
}

impl PayloadExampleModel {
    pub fn from_model(pe: &Vec<PayloadExampleModel>) -> Result<Vec<(Name, Payload)>, ApiError> {
        let mut result = Vec::with_capacity(pe.len());
        for p in pe {
            result.push((Name::try_from(&p.name)?, Payload::new(p.payload.clone())));
        }
        Ok(result)
    }
}

impl TryInto<(Name, Payload)> for &PayloadExampleModel {
    type Error = ApiError;

    fn try_into(self) -> Result<(Name, Payload), Self::Error> {
        Ok((
            Name::try_from(&self.name)?,
            Payload::new(self.payload.clone()),
        ))
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateMessageTypeRequest {
    pub code: String,
    pub name: String,
    pub vars: Option<HashMap<String, String>>,
    pub schema: Value,
    pub enabled: bool,
    pub payload_examples: Vec<PayloadExampleModel>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateMessageTypeRequest {
    pub name: String,
    pub vars: Option<HashMap<String, String>>,
    pub schema: Value,
    pub enabled: bool,
    pub payload_examples: Vec<PayloadExampleModel>,
}
