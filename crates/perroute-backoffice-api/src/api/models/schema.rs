use std::ops::Deref;

use crate::{
    api::response::{CollectionResourceModel, Links, ResourceBuilder, SingleResourceModel},
    links::{Linkrelation, ResourceLink},
};
use perroute_commons::types::{json_schema::JsonSchema, vars::Vars};
use perroute_storage::models::{
    message_type::MessageType,
    schema::{Schema, Version},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateSchemaRequest {
    pub schema: JsonSchema,
    pub enabled: bool,
    pub vars: Vars,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSchemaRequest {
    pub schema: JsonSchema,
    pub enabled: bool,
    pub vars: Vars,
}

#[derive(Debug, Serialize, Clone)]
pub struct SchemaResource {
    schema: JsonSchema,
    version: Version,
    published: bool,
    enabled: bool,
}

impl From<Schema> for SchemaResource {
    fn from(value: Schema) -> Self {
        Self {
            schema: value.schema().deref().clone(),
            version: *value.version(),
            published: *value.published(),
            enabled: *value.enabled(),
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
                    ResourceLink::Schema(*self.message_type_id(), *self.id()),
                )
                .add(
                    Linkrelation::Schemas,
                    ResourceLink::Schemas(*self.message_type_id()),
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
                .add(Linkrelation::Self_, ResourceLink::Schemas(*self.0.id()))
                .as_url_map(req),
        }
    }
}
