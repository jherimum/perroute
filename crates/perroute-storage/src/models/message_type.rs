use std::ops::Deref;

use bon::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{id::Id, name::Name, vars::Vars, Code, Payload, Schema, Timestamp};
use serde_json::Value;
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Builder, Getters, Setters)]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct MessageType {
    #[setters(skip)]
    id: Id,
    #[setters(skip)]
    code: Code,

    name: Name,

    vars: Option<Vars>,

    schema: Schema,

    enabled: bool,

    #[setters(skip)]
    created_at: Timestamp,

    updated_at: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Builder, Getters, Setters)]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct PayloadExample {
    id: Id,
    message_type_id: Id,
    name: Name,
    payload: Payload,
}

impl From<&PayloadExample> for (String, Value) {
    fn from(value: &PayloadExample) -> Self {
        (value.name().to_string(), value.payload().deref().clone())
    }
}
