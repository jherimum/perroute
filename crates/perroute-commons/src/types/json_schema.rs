use super::payload::Payload;
use jsonschema::{
    output::{ErrorDescription, OutputUnit},
    JSONSchema,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::Type;
use std::{borrow::Cow, collections::VecDeque, fmt::Display, ops::Deref};
use tap::TapFallible;
use validator::ValidationError;

#[derive(thiserror::Error, Debug)]
pub struct InvalidSchemaError(String);

impl Display for InvalidSchemaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(thiserror::Error, Debug)]
pub struct InvalidPayloadError(VecDeque<OutputUnit<ErrorDescription>>);

impl Display for InvalidPayloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Eq, Deserialize, Type)]
#[serde(transparent)]
#[sqlx(transparent)]
pub struct JsonSchema(serde_json::Value);

impl Default for JsonSchema {
    fn default() -> Self {
        Self(json!({}))
    }
}

impl From<&JsonSchema> for Value {
    fn from(value: &JsonSchema) -> Self {
        value.0.clone()
    }
}

impl TryFrom<Value> for JsonSchema {
    type Error = InvalidSchemaError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        JSONSchema::compile(&value)
            .tap_err(|e| tracing::error!("Invalid schema: {e}"))
            .map_err(|e| InvalidSchemaError(e.to_string()))?;
        Ok(Self(value))
    }
}

impl Deref for JsonSchema {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl JsonSchema {
    pub fn validate_payload(&self, payload: &Payload) -> Result<(), InvalidPayloadError> {
        let compiled = JSONSchema::compile(&self.0).unwrap();
        match compiled.apply(payload).basic() {
            jsonschema::output::BasicOutput::Valid(_) => Ok(()),
            jsonschema::output::BasicOutput::Invalid(e) => Err(InvalidPayloadError(e)),
        }
    }

    pub fn validate(value: &Value) -> Result<(), ValidationError> {
        if Self::try_from(value.clone()).is_err() {
            return Err(ValidationError {
                code: Cow::Borrowed("schema"),
                message: Some(Cow::Borrowed("Invalid schema")),
                params: Default::default(),
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_default() {
        JSONSchema::compile(&json!({
          "$schema": "http://json-schema.org/draft-04/schema#",
          "type": "object",
          "properties": {
            "order_number": {
              "type": "string1"
            }
          },
          "requir1ed": [
            "order_numqber"
          ]
        }))
        .unwrap();

        let schema = JsonSchema::default();
        assert_eq!(schema, JsonSchema(json!({})));
    }

    // #[test]
    // fn test_try_from() {
    //     let schema = JsonSchema(json!({
    //       "$schema": "http://json-schema.org/draft-04/schema#",
    //       "type": "object",
    //       "properties": {
    //         "order_number": {
    //           "type": "string"
    //         }
    //       },
    //       "required": [
    //         "order_number"
    //       ]
    //     }));

    //     assert!(schema.is_ok());

    //     let schema = JsonSchema(json!({
    //       "$schema": "http://json-schema.org/draft-04/schema#",
    //       "type": "object",
    //       "properties": {
    //         "order_number": {
    //           "type": "str"
    //         }
    //       },
    //       "required": [
    //         "order_number"
    //       ]
    //     }));

    //     assert!(schema.is_err());
    // }

    // #[test]
    // fn test_validate() {
    //     let schema = JsonSchema();

    //     assert!(schema.is_ok());

    //     let payload = Payload::new(json!({
    //       "order_number": "123"
    //     }));

    //     let schema = schema.unwrap();

    //     assert!(schema.validate(&payload).is_ok());

    //     let payload = Payload::new(json!({
    //       "order_number": 123
    //     }));

    //     assert!(schema.validate(&payload).is_err());
    // }
}
