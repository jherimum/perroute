use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use sqlx::Type;
use validator::{Validate, ValidationErrors};

#[derive(Debug, thiserror::Error)]
pub enum PropertiesError {
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),

    #[error(transparent)]
    ValidationError(#[from] ValidationErrors),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default, Type)]
pub struct Properties(Value);

impl Properties {
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
