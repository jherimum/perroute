use crate::api::response::{Resource, SingleResource};
use perroute_commons::types::id::Id;
use perroute_storage::models::schema::Schema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct CreateSchemaRequest {
    pub message_type_id: Id,
    pub schema: Value,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSchemaRequest {
    pub schema: Value,
}

#[derive(Debug, Serialize, Clone)]
pub struct SchemaResource {}

impl Resource for SchemaResource {}

impl From<SchemaResource> for Schema {
    fn from(value: SchemaResource) -> Self {
        todo!()
    }
}

impl From<Schema> for SingleResource<SchemaResource> {
    fn from(value: Schema) -> Self {
        todo!()
    }
}
