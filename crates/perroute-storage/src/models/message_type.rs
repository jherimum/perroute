use std::ops::Deref;

use bon::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{
    code::Code, id::Id, name::Name, schema::Schema, vars::Vars, Payload,
    Timestamp,
};
use serde_json::Value;
use sqlx::{prelude::FromRow, types::Json};

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Builder, Getters, Setters)]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct MessageType {
    #[setters(skip)]
    id: Id,
    #[setters(skip)]
    code: Code,

    name: Name,

    vars: Json<Vars>,

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

impl perroute_commons::types::entity::Entity for MessageType {
    fn id(&self) -> &Id {
        &self.id
    }
}
