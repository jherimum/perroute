use crate::{
    api::response::{CollectionResourceModel, Links, ResourceBuilder, SingleResourceModel},
    links::{Linkrelation, ResourceLink},
};
use anyhow::{Context, Result};
use perroute_commons::types::{id::Id, json_schema::JsonSchema, vars::Vars};
use perroute_storage::models::schema::Schema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Default)]
#[serde(default)]
pub struct CreateSchemaRequest {
    #[validate(required)]
    #[validate(custom = "Id::validate")]
    message_type_id: Option<String>,

    #[validate(required)]
    #[validate(custom = "JsonSchema::validate")]
    value: Option<Value>,

    vars: Option<HashMap<String, String>>,
}

impl CreateSchemaRequest {
    pub fn message_type_id(&self) -> Result<Id> {
        self.message_type_id
            .clone()
            .context("missing message type id")?
            .try_into()
            .context("invalid message type id")
    }

    pub fn value(&self) -> Result<JsonSchema> {
        self.value
            .clone()
            .context("missing value")?
            .try_into()
            .context("Invalid schame")
    }

    pub fn vars(&self) -> Result<Vars> {
        Ok(self.vars.clone().map(Into::into).unwrap_or_default())
    }
}

#[derive(Debug, Deserialize, Validate, Default)]
#[serde(default)]
pub struct UpdateSchemaRequest {
    #[validate(custom = "JsonSchema::validate")]
    value: Option<Value>,
    enabled: Option<bool>,
    vars: Option<HashMap<String, String>>,
}

impl UpdateSchemaRequest {
    pub fn value(&self) -> Result<Option<JsonSchema>> {
        Ok(self.value.clone().map(TryInto::try_into).transpose()?)
    }

    pub fn enabled(&self) -> Result<Option<bool>> {
        Ok(self.enabled)
    }

    pub fn vars(&self) -> Option<Vars> {
        self.vars.clone().map(|v| v.into())
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct SchemaResource {
    value: Value,
    version: i32,
    published: bool,
    enabled: bool,
}

impl From<&Schema> for SchemaResource {
    fn from(value: &Schema) -> Self {
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
            data: Some(self.into()),
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
