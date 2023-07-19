use crate::{
    api::response::{CollectionResourceModel, Links, ResourceBuilder, SingleResourceModel},
    links::{Linkrelation, ResourceLink},
};
use perroute_commons::types::json_schema::JsonSchema;
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

impl From<Schema> for SchemaResource {
    fn from(value: Schema) -> Self {
        Self {
            schema: value.schema().clone(),
            version: *value.version(),
            published: *value.published(),
        }
    }
}

impl ResourceBuilder<SingleResourceModel<SchemaResource>> for Schema {
    fn build(&self, req: &actix_web::HttpRequest) -> SingleResourceModel<SchemaResource> {
        SingleResourceModel {
            data: Some(self.clone().into()),
            links: Links::default()
                .add(
                    Linkrelation::Self_,
                    ResourceLink::Schema(*self.channel_id(), *self.message_type_id(), *self.id()),
                )
                .add(
                    Linkrelation::Schemas,
                    ResourceLink::Schemas(*self.channel_id(), *self.message_type_id()),
                )
                .as_url_map(req),
        }
    }
}

impl ResourceBuilder<CollectionResourceModel<SchemaResource>> for (MessageType, Vec<Schema>) {
    fn build(&self, req: &actix_web::HttpRequest) -> CollectionResourceModel<SchemaResource> {
        CollectionResourceModel {
            data: self.1.iter().map(|s| s.build(req)).collect(),
            links: Links::default()
                .add(
                    Linkrelation::Self_,
                    ResourceLink::Schemas(*self.0.channel_id(), *self.0.id()),
                )
                .as_url_map(req),
        }
    }
}
