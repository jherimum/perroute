use std::collections::HashMap;

use crate::{
    api::response::{CollectionResourceModel, Links, ResourceBuilder, SingleResourceModel},
    links::{Linkrelation, ResourceLink},
};
use perroute_storage::models::schema::Schema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Default)]
#[serde(default)]
pub struct CreateSchemaRequest {
    #[validate(required)]
    #[validate(custom = "perroute_commons::types::id::Id::validate")]
    pub message_type_id: Option<String>,

    #[validate(required)]
    #[validate(custom = "perroute_commons::types::json_schema::JsonSchema::validate")]
    pub value: Option<Value>,

    pub vars: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Validate, Default)]
#[serde(default)]
pub struct UpdateSchemaRequest {
    #[validate(custom = "perroute_commons::types::json_schema::JsonSchema::validate")]
    pub value: Option<Value>,
    pub enabled: Option<bool>,
    pub vars: Option<HashMap<String, String>>,
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
                .add(Linkrelation::Self_, ResourceLink::Schema(*self.id()))
                .add(Linkrelation::Schemas, ResourceLink::Schemas)
                .add(
                    Linkrelation::MessageType,
                    ResourceLink::MessageType(*self.message_type_id()),
                )
                .as_url_map(req),
        }
    }
}

impl ResourceBuilder<CollectionResourceModel<SchemaResource>> for Vec<Schema> {
    fn build(&self, req: &actix_web::HttpRequest) -> CollectionResourceModel<SchemaResource> {
        CollectionResourceModel {
            data: self.iter().map(|s| s.build(req)).collect(),
            links: Links::default()
                .add(Linkrelation::Self_, ResourceLink::Schemas)
                .as_url_map(req),
        }
    }
}
