use serde::Serialize;
use serde_json::{json, Value};
use sqlx::Type;
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, Type, Serialize)]
#[sqlx(transparent)]
pub struct Payload(Value);

impl Default for Payload {
    fn default() -> Self {
        Self(json!({}))
    }
}

impl Payload {
    pub const fn new(value: Value) -> Self {
        Self(value)
    }
}

impl From<Value> for Payload {
    fn from(value: Value) -> Self {
        Self(value)
    }
}

impl Deref for Payload {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
