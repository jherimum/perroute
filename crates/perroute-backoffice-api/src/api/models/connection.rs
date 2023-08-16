use crate::{
    api::response::{CollectionResourceModel, Links, ResourceBuilder, SingleResourceModel},
    links::{Linkrelation, ResourceLink},
};
use actix_web::HttpRequest;
use derive_getters::Getters;
use perroute_commons::types::{id::Id, properties::Properties};
use perroute_connectors::types::ConnectorPluginId;
use perroute_storage::models::connection::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Getters)]
pub struct CreateConnectionRequest {
    name: String,
    plugin_id: ConnectorPluginId,
    properties: Properties,
}

#[derive(Debug, Deserialize, Clone, Getters)]
pub struct UpdateConnectionRequest {
    name: String,
    properties: Properties,
    enabled: bool,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Eq)]
pub struct ConnectionResource {
    pub id: Id,
    pub name: String,
    pub properties: Properties,
    pub enabled: bool,
    pub plugin_id: ConnectorPluginId,
}

impl From<Connection> for ConnectionResource {
    fn from(value: Connection) -> Self {
        Self {
            id: *value.id(),
            name: value.name().clone(),
            properties: value.properties().clone(),
            enabled: *value.enabled(),
            plugin_id: *value.plugin_id(),
        }
    }
}

impl ResourceBuilder<SingleResourceModel<ConnectionResource>> for Connection {
    fn build(&self, req: &HttpRequest) -> SingleResourceModel<ConnectionResource> {
        SingleResourceModel {
            data: Some(ConnectionResource::from(self.clone())),
            links: Links::default()
                .add(Linkrelation::Self_, ResourceLink::Connection(*self.id()))
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
