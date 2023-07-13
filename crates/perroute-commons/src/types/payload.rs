use serde::Serialize;
use serde_json::Value;
use sqlx::Type;
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, Type, Serialize)]
#[sqlx(transparent)]
pub struct Payload(Value);

impl Payload {
    pub const fn new(value: Value) -> Self {
        Self(value)
    }
}

impl Deref for Payload {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
