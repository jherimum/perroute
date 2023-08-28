use crate::{
    api::response::{CollectionResourceModel, Links, ResourceBuilder, SingleResourceModel},
    links::{Linkrelation, ResourceLink},
};
use actix_web::HttpRequest;
use perroute_storage::models::route::Route;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;

#[derive(Debug, serde::Deserialize, Clone, Validate, Default)]
#[serde(default)]
pub struct CreateRouteRequest {
    #[validate(custom = "perroute_commons::types::id::Id::validate")]
    pub channel_id: String,

    #[validate(custom = "perroute_commons::types::id::Id::validate")]
    pub schema_id: String,

    #[validate(custom = "perroute_commons::types::properties::Properties::validate")]
    pub properties: Option<Value>,
}

#[derive(Debug, serde::Deserialize, Clone, Validate, Default)]
#[serde(default)]
pub struct UpdateRouteRequest {
    #[validate(custom = "perroute_commons::types::properties::Properties::validate")]
    pub properties: Option<Value>,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Eq)]
pub struct RouteResource {
    pub id: String,
    pub properties: Value,
}

impl From<Route> for RouteResource {
    fn from(value: Route) -> Self {
        Self {
            id: value.id().into(),
            properties: value.properties().into(),
        }
    }
}

impl ResourceBuilder<SingleResourceModel<RouteResource>> for Route {
    fn build(&self, req: &HttpRequest) -> SingleResourceModel<RouteResource> {
        SingleResourceModel {
            data: Some(RouteResource::from(self.clone())),
            links: Links::default()
                .add(Linkrelation::Self_, ResourceLink::Route(*self.id()))
                .add(Linkrelation::Routes, ResourceLink::Routes)
                .add(
                    Linkrelation::Channel,
                    ResourceLink::Channel(*self.channel_id()),
                )
                .add(
                    Linkrelation::Schema,
                    ResourceLink::Schema(*self.message_type_id(), *self.schema_id()),
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
