use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use sqlx::Type;
use std::borrow::Cow;
use std::ops::Deref;
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Debug, thiserror::Error)]
pub enum PropertiesError {
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),

    #[error(transparent)]
    ValidationError(#[from] ValidationErrors),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default, Type)]
#[sqlx(transparent)]
pub struct Properties(Value);

impl Deref for Properties {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<Value> for &Properties {
    fn into(self) -> Value {
        self.0.clone()
    }
}

impl From<&Value> for Properties {
    fn from(value: &Value) -> Self {
        Self(value.clone())
    }
}

impl Properties {
    pub fn validate(value: &Value) -> Result<(), ValidationError> {
        if value.is_object() {
            return Ok(());
        }
        return Err(ValidationError {
            code: Cow::Borrowed("properties"),
            message: Some(Cow::Borrowed("Invalid properties")),
            params: Default::default(),
        });
    }

    pub fn new(value: Value) -> Self {
        Self(value)
    }

    pub fn merge(&self, other: &Self) -> Self {
        let mut base = self.0.clone();
        merge_values(&mut base, other.0.clone());
        Self(base)
    }

    pub fn from_value<T: DeserializeOwned + Validate>(&self) -> Result<T, PropertiesError> {
        let t = serde_json::from_value::<T>(self.0.clone())?;
        t.validate()?;
        Ok(t)
    }
}

fn merge_values(a: &mut Value, b: Value) {
    match (a, b) {
        (a @ &mut Value::Object(_), Value::Object(b)) => {
            let a = a.as_object_mut().unwrap();
            for (k, v) in b {
                merge_values(a.entry(k).or_insert(Value::Null), v);
            }
        }
        (a, b) => *a = b,
    }
}
