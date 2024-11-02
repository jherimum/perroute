use std::ops::Deref;

use serde::Serialize;
use serde_json::Value;
use sqlx::prelude::Type;

#[derive(Debug, Clone, PartialEq, Eq, Type, Serialize)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct Schema(Value);

impl Schema {
    pub fn new(value: Value) -> Self {
        Self(value)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct InvalidSchemaError(String);

impl TryFrom<&Value> for Schema {
    type Error = InvalidSchemaError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        Ok(Self(value.to_owned()))
    }
}

impl Deref for Schema {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
