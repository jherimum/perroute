use super::payload::Payload;
use jsonschema::JSONSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::Type;
use std::ops::Deref;

#[derive(Debug, thiserror::Error)]
pub enum JsonSchemaError {
    #[error("Invalid schema")]
    InvalidSchema,

    #[error("Invalid input")]
    ValidationError,
}

#[derive(Debug, Clone, PartialEq, Serialize, Eq, Deserialize, Type)]
#[serde(transparent)]
pub struct JsonSchema(serde_json::Value);

impl Default for JsonSchema {
    fn default() -> Self {
        Self(json!({}))
    }
}

// impl TryFrom<Value> for JsonSchema {
//     type Error = JsonSchemaError;

//     fn try_from(value: Value) -> Result<Self, Self::Error> {
//         JSONSchema::compile(&value)
//             .tap_err(|e| tracing::error!("Error: {e}"))
//             .map_err(|_| JsonSchemaError::InvalidSchema)?;
//         Ok(Self(value))
//     }
// }

impl Deref for JsonSchema {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl JsonSchema {
    pub fn validate(&self, payload: &Payload) -> Result<(), JsonSchemaError> {
        let compiled = JSONSchema::compile(&self.0).unwrap();
        match compiled.apply(payload).basic() {
            jsonschema::output::BasicOutput::Valid(_) => Ok(()),
            jsonschema::output::BasicOutput::Invalid(_) => Err(JsonSchemaError::ValidationError),
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_default() {
        JSONSchema::compile(&json!({})).unwrap();

        let schema = JsonSchema::default();
        assert_eq!(schema, JsonSchema(json!({})));
    }

    #[test]
    fn test_try_from() {
        let schema = JsonSchema::try_from(json!({
          "$schema": "http://json-schema.org/draft-04/schema#",
          "type": "object",
          "properties": {
            "order_number": {
              "type": "string"
            }
          },
          "required": [
            "order_number"
          ]
        }));

        assert!(schema.is_ok());

        let schema = JsonSchema::try_from(json!({
          "$schema": "http://json-schema.org/draft-04/schema#",
          "type": "object",
          "properties": {
            "order_number": {
              "type": "str"
            }
          },
          "required": [
            "order_number"
          ]
        }));

        assert!(schema.is_err());
    }

    #[test]
    fn test_validate() {
        let schema = JsonSchema::try_from(json!({
          "$schema": "http://json-schema.org/draft-04/schema#",
          "type": "object",
          "properties": {
            "order_number": {
              "type": "string"
            }
          },
          "required": [
            "order_number"
          ]
        }));

        assert!(schema.is_ok());

        let payload = Payload::new(json!({
          "order_number": "123"
        }));

        let schema = schema.unwrap();

        assert!(schema.validate(&payload).is_ok());

        let payload = Payload::new(json!({
          "order_number": 123
        }));

        assert!(schema.validate(&payload).is_err());
    }
}
 */
