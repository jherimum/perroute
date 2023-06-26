use perroute_commons::types::id::Id;
use perroute_storage::models::schema::Schema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::ops::Deref;

#[derive(Clone, Deserialize, Debug)]
pub struct CreateSchemaRequest {
    pub schema: Value,
}

#[derive(Clone, Deserialize, Debug)]
pub struct UpdateSchemaRequest {
    pub schema: Value,
}

#[derive(Clone, Serialize)]
pub struct SchemaResource {
    pub id: Id,
    pub version: i32,
    pub schema: Value,
    pub message_type_id: Id,
}

impl From<Schema> for SchemaResource {
    fn from(value: Schema) -> Self {
        SchemaResource {
            id: *value.id(),
            version: (*value.version()).into(),
            schema: (*value.schema().deref()).clone(),
            message_type_id: *value.message_type_id(),
        }
    }
}
impl From<&Schema> for SchemaResource {
    fn from(value: &Schema) -> Self {
        value.clone().into()
    }
}
