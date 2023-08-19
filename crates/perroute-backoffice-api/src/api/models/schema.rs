use std::collections::HashMap;

use crate::{
    api::response::{CollectionResourceModel, Links, ResourceBuilder, SingleResourceModel},
    links::{Linkrelation, ResourceLink},
};
use perroute_storage::models::{message_type::MessageType, schema::Schema};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Default)]
pub struct CreateSchemaRequest {
    #[validate(custom = "perroute_commons::types::json_schema::JsonSchema::validate")]
    pub value: Value,
    pub enabled: bool,
    pub vars: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateSchemaRequest {
    #[validate(custom = "perroute_commons::types::json_schema::JsonSchema::validate")]
    pub value: Value,
    pub enabled: bool,
    pub vars: HashMap<String, String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct SchemaResource {
    value: Value,
    version: i32,
    published: bool,
    enabled: bool,
}

impl From<Schema> for SchemaResource {
    fn from(value: Schema) -> Self {
        Self {
            value: value.value().into(),
            version: value.version().into(),
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
