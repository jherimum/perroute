use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct Properties(Value);

impl Properties {
    pub fn new(value: Value) -> Self {
        Self(value)
    }
}

impl Properties {
    pub fn from_value<T: DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_value(self.0.clone())
    }
}
