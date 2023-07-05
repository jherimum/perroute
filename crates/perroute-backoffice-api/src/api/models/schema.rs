use crate::api::{
    response::{CollectionResource, Resource, SingleResource},
    Linkrelation, ResourceLink,
};
use perroute_commons::types::{id::Id, json_schema::JsonSchema};
use perroute_storage::models::{
    message_type::MessageType,
    schema::{Schema, Version},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct CreateSchemaRequest {
    pub schema: Value,
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

impl From<(MessageType, Vec<Schema>)> for CollectionResource<SchemaResource> {
    fn from(value: (MessageType, Vec<Schema>)) -> Self {
        CollectionResource::default()
            .with_link(Linkrelation::Self_, ResourceLink::Schemas(*value.0.id()))
            .with_resources(value.1.into_iter().map(Schema::into).collect())
    }
}
