use crate::rest::{error::ApiError, models::resource::ResourceModel};
use bon::Builder;
use chrono::NaiveDateTime;
use perroute_commons::types::{
    code::Code, id::Id, name::Name, schema::Schema, vars::Vars, Payload,
};
use perroute_storage::models::message_type::{MessageType, PayloadExample};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, ops::Deref};
use validator::Validate;

#[derive(Debug, Deserialize)]
pub struct MessageTypePath(String);

impl MessageTypePath {
    pub fn id(&self) -> Id {
        Id::from(&self.0)
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
    pub vars: HashMap<String, String>,
    pub schema: Value,
    pub enabled: bool,
    pub payload_examples: Vec<(String, Value)>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<(MessageType, Vec<PayloadExample>)> for MessageTypeModel {
    fn from(value: (MessageType, Vec<PayloadExample>)) -> Self {
        MessageTypeModel::builder()
            .id(value.0.id().to_string())
            .code(value.0.code().to_string())
            .name(value.0.name().to_string())
            .vars((&**value.0.vars()).into())
            .schema(value.0.schema().deref().clone())
            .enabled(*value.0.enabled())
            .created_at(**value.0.created_at())
            .updated_at(**value.0.updated_at())
            .payload_examples(value.1.iter().map(Into::into).collect())
            .build()
    }
}

impl From<(MessageType, Vec<PayloadExample>)>
    for ResourceModel<MessageTypeModel>
{
    fn from(value: (MessageType, Vec<PayloadExample>)) -> Self {
        ResourceModel::new(value.into())
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct PayloadExampleModel {
    name: String,
    payload: Value,
}

impl PayloadExampleModel {
    pub fn from_model(
        pe: &Vec<PayloadExampleModel>,
    ) -> Result<Vec<(Name, Payload)>, ApiError> {
        let mut result = Vec::with_capacity(pe.len());
        for p in pe {
            result.push((
                Name::try_from(p.name.as_str())?,
                Payload::new(p.payload.clone()),
            ));
        }
        Ok(result)
    }
}

impl TryInto<(Name, Payload)> for &PayloadExampleModel {
    type Error = ApiError;

    fn try_into(self) -> Result<(Name, Payload), Self::Error> {
        Ok((
            Name::try_from(self.name.as_str())?,
            Payload::new(self.payload.clone()),
        ))
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateMessageTypeRequest {
    code: String,
    name: String,
    vars: HashMap<String, String>,
    schema: Value,
    enabled: bool,
    payload_examples: Vec<PayloadExampleModel>,
}

impl CreateMessageTypeRequest {
    pub fn code(&self) -> Result<Code, ApiError> {
        Ok(Code::try_from(&self.code)?)
    }

    pub fn name(&self) -> Result<Name, ApiError> {
        Ok(Name::try_from(self.name.as_str())?)
    }

    pub fn vars(&self) -> Vars {
        Vars::from(&self.vars)
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn payload_examples(&self) -> Result<Vec<(Name, Payload)>, ApiError> {
        PayloadExampleModel::from_model(&self.payload_examples)
    }
    pub fn schema(&self) -> Result<Schema, ApiError> {
        Ok(Schema::try_from(&self.schema)?)
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateMessageTypeRequest {
    name: String,
    vars: HashMap<String, String>,
    schema: Value,
    enabled: bool,
    payload_examples: Vec<PayloadExampleModel>,
}

impl UpdateMessageTypeRequest {
    pub fn name(&self) -> Result<Name, ApiError> {
        Ok(Name::try_from(self.name.as_str())?)
    }

    pub fn vars(&self) -> Vars {
        Vars::from(&self.vars)
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn payload_examples(&self) -> Result<Vec<(Name, Payload)>, ApiError> {
        PayloadExampleModel::from_model(&self.payload_examples)
    }

    pub fn schema(&self) -> Result<Schema, ApiError> {
        Ok(Schema::try_from(&self.schema)?)
    }
}
