use crate::api::{
    response::{Resource, SingleResource},
    Linkrelation, ResourceLink,
};
use perroute_commons::types::{id::Id, json_schema::JsonSchema};
use perroute_storage::models::schema::{Schema, Version};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct CreateSchemaRequest {
    pub schema: Value,
    pub message_type_id: Id,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSchemaRequest {
    pub schema: Value,
}

#[derive(Debug, Serialize, Clone)]
pub struct SchemaResource {
    schema: JsonSchema,
    version: Version,
    published: bool,
}

impl Resource for SchemaResource {}

impl From<Schema> for SchemaResource {
    fn from(value: Schema) -> Self {
        SchemaResource {
            schema: value.schema().to_owned(),
            version: value.version().to_owned(),
            published: value.published().to_owned(),
        }
    }
}

impl From<Schema> for SingleResource<SchemaResource> {
    fn from(value: Schema) -> Self {
        SingleResource::default()
            .with_data(value.clone().into())
            .with_link(
                Linkrelation::Self_,
                ResourceLink::Schema(*value.channel_id(), *value.id()),
            )
            .with_link(
                Linkrelation::Schemas,
                ResourceLink::Schemas(*value.channel_id()),
            )
    }
}
