use crate::{
    api::response::{CollectionResourceModel, Links, ResourceBuilder, SingleResourceModel},
    links::{Linkrelation, ResourceLink},
};
use actix_web::HttpRequest;
use anyhow::{Context, Result};
use perroute_commons::types::{id::Id, properties::Properties};
use perroute_storage::models::route::Route;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;

#[derive(Debug, serde::Deserialize, Clone, Validate, Default)]
#[serde(default)]
pub struct CreateRouteRequest {
    #[validate(required)]
    #[validate(custom = "Id::validate")]
    channel_id: Option<String>,

    #[validate(required)]
    #[validate(custom = "Id::validate")]
    schema_id: Option<String>,

    #[validate(required)]
    #[validate(custom = "Properties::validate")]
    properties: Option<Value>,
}

impl CreateRouteRequest {
    pub fn channel_id(&self) -> Result<Id> {
        Ok(self
            .channel_id
            .clone()
            .context("missing channel id")?
            .try_into()
            .context("invalid channel id")?)
    }

    pub fn schema_id(&self) -> Result<Id> {
        Ok(self
            .schema_id
            .clone()
            .context("missing schema id")?
            .try_into()
            .context("invalid schema id")?)
    }

    pub fn properties(&self) -> Result<Properties> {
        Ok(self
            .properties
            .clone()
            .context("missing properties")?
            .into())
    }
}

#[derive(Debug, serde::Deserialize, Clone, Validate, Default)]
#[serde(default)]
pub struct UpdateRouteRequest {
    #[validate(custom = "Properties::validate")]
    properties: Option<Value>,
}

impl UpdateRouteRequest {
    pub fn properties(&self) -> Result<Option<Properties>> {
        Ok(self.properties.clone().map(Into::into))
    }
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Eq)]
pub struct RouteResource {
    pub id: String,
    pub properties: Value,
}

impl From<&Route> for RouteResource {
    fn from(value: &Route) -> Self {
        Self {
            id: value.id().into(),
            properties: value.properties().into(),
        }
    }
}

impl ResourceBuilder<SingleResourceModel<RouteResource>> for Route {
    fn build(&self, req: &HttpRequest) -> SingleResourceModel<RouteResource> {
        SingleResourceModel {
            data: Some(self.into()),
            links: Links::default()
                .add(Linkrelation::Self_, ResourceLink::Route(*self.id()))
                .add(Linkrelation::Routes, ResourceLink::Routes)
                .add(
                    Linkrelation::Channel,
                    ResourceLink::Channel(*self.channel_id()),
                )
                .add(
                    Linkrelation::Schema,
                    ResourceLink::Schema(*self.schema_id()),
                )
                .as_url_map(req),
        }
    }
}

impl ResourceBuilder<CollectionResourceModel<RouteResource>> for Vec<Route> {
    fn build(&self, req: &HttpRequest) -> CollectionResourceModel<RouteResource> {
        CollectionResourceModel {
            data: self.iter().map(|c| c.build(req)).collect(),
            links: Links::default()
                .add(Linkrelation::Self_, ResourceLink::Routes)
                .as_url_map(req),
        }
    }
}
