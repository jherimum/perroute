use serde::Deserialize;
use serde_json::Value;

#[derive(Clone, Deserialize, Debug)]
pub struct CreateSchemaRequest {
    pub schema: Value,
}

#[derive(Clone, Deserialize, Debug)]
pub struct UpdateSchemaRequest {
    pub schema: Value,
}
