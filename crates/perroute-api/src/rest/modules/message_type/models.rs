use crate::rest::{error::ApiError, models::ResourceModel};
use bon::Builder;
use chrono::NaiveDateTime;
use perroute_commons::types::{id::Id, name::Name, vars::Vars, Code, Payload, Schema};
use perroute_storage::models::message_type::{MessageType, PayloadExample};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, ops::Deref, vec};
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

impl TryInto<(Name, Payload)> for &PayloadExampleModel {
    type Error = ApiError;

    fn try_into(self) -> Result<(Name, Payload), Self::Error> {
        Ok((
            Name::try_from(&self.name)?,
            Payload::new(self.payload.clone()),
        ))
    }
}

impl PayloadExampleModel {
    pub fn payload(&self) -> Payload {
        Payload::new(self.payload.clone())
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

impl CreateMessageTypeRequest {
    pub fn code(&self) -> Result<Code, ApiError> {
        Code::try_from(self.code.clone()).map_err(|_| ApiError::BadRequest)
    }

    pub fn vars(&self) -> Option<Vars> {
        self.vars.as_ref().map(|v| Vars::new(v.clone()))
    }

    pub fn schema(&self) -> Schema {
        Schema::new(self.schema.clone())
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn examples(&self) -> Result<Vec<(Name, Payload)>, ApiError> {
        let mut r = vec![];
        for e in &self.payload_examples {
            r.push((Name::try_from(&e.name)?, e.payload()));
        }

        Ok(r)
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateMessageTypeRequest {
    pub name: String,
    pub vars: Option<HashMap<String, String>>,
    pub schema: Value,
    pub enabled: bool,
    pub payload_examples: Vec<PayloadExampleModel>,
}

impl UpdateMessageTypeRequest {
    pub fn examples(&self) -> Result<Vec<(Name, Payload)>, ApiError> {
        let mut r = vec![];
        for e in &self.payload_examples {
            r.push(e.try_into()?);
        }
        Ok(r)
    }

    pub fn vars(&self) -> Option<Vars> {
        self.vars.as_ref().map(|v| Vars::new(v.clone()))
    }

    pub fn schema(&self) -> Schema {
        Schema::new(self.schema.clone())
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }
}
