use crate::{
    api::response::{CollectionResourceModel, Links, ResourceBuilder, SingleResourceModel},
    links::{Linkrelation, ResourceLink},
};
use actix_web::HttpRequest;
use derive_getters::Getters;
use perroute_storage::models::connection::Connection;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;

#[derive(Debug, Deserialize, Clone, Getters, Validate, Default)]
#[serde(default)]
pub struct CreateConnectionRequest {
    #[validate(custom = "perroute_commons::types::name::validate")]
    name: String,

    #[validate(custom = "perroute_connectors::types::ConnectorPluginId::validate")]
    plugin_id: String,

    #[validate(custom = "perroute_commons::types::properties::Properties::validate")]
    properties: Value,
}

#[derive(Debug, Deserialize, Clone, Getters, Validate, Default)]
#[serde(default)]
pub struct UpdateConnectionRequest {
    #[validate(custom = "perroute_commons::types::name::validate")]
    name: String,

    #[validate(custom = "perroute_commons::types::properties::Properties::validate")]
    properties: Value,

    enabled: bool,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Eq)]
pub struct ConnectionResource {
    pub id: String,
    pub name: String,
    pub properties: Value,
    pub enabled: bool,
    pub plugin_id: String,
}

impl From<Connection> for ConnectionResource {
    fn from(value: Connection) -> Self {
        Self {
            id: value.id().into(),
            name: value.name().into(),
            properties: value.properties().into(),
            enabled: *value.enabled(),
            plugin_id: value.plugin_id().into(),
        }
    }
}

impl ResourceBuilder<SingleResourceModel<ConnectionResource>> for Connection {
    fn build(&self, req: &HttpRequest) -> SingleResourceModel<ConnectionResource> {
        SingleResourceModel {
            data: Some(ConnectionResource::from(self.clone())),
            links: Links::default()
                .add(Linkrelation::Self_, ResourceLink::Connection(*self.id()))
                .add(
                    Linkrelation::Plugin,
                    ResourceLink::Plugin(*self.plugin_id()),
                )
                .add(Linkrelation::Connections, ResourceLink::Connections)
                .as_url_map(req),
        }
    }
}

impl ResourceBuilder<CollectionResourceModel<ConnectionResource>> for Vec<Connection> {
    fn build(&self, req: &HttpRequest) -> CollectionResourceModel<ConnectionResource> {
        CollectionResourceModel {
            data: self.iter().map(|c| c.build(req)).collect(),
            links: Links::default()
                .add(Linkrelation::Self_, ResourceLink::Connections)
                .as_url_map(req),
        }
    }
}
