use crate::{
    api::response::{CollectionResourceModel, Links, ResourceBuilder, SingleResourceModel},
    links::{Linkrelation, ResourceLink},
};
use actix_web::HttpRequest;
use anyhow::{Context, Result};
use perroute_commons::types::properties::Properties;
use perroute_connectors::types::plugin_id::ConnectorPluginId;
use perroute_storage::models::connection::Connection;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;

#[derive(Debug, Deserialize, Clone, Validate, Default)]
#[serde(default)]
pub struct CreateConnectionRequest {
    #[validate(required)]
    #[validate(custom = "perroute_commons::types::name::validate")]
    name: Option<String>,

    #[validate(required)]
    #[validate(custom = "ConnectorPluginId::validate")]
    plugin_id: Option<String>,

    #[validate(required)]
    #[validate(custom = "Properties::validate")]
    properties: Option<Value>,
}

impl CreateConnectionRequest {
    pub fn plugin_id(&self) -> Result<ConnectorPluginId> {
        self.plugin_id
            .clone()
            .context("Missing plugin id")?
            .try_into()
            .context("Invalid plugin id")
    }

    pub fn name(&self) -> Result<String> {
        self.name.clone().context("Missing name")
    }

    pub fn properties(&self) -> Result<Properties> {
        self.properties
            .clone()
            .context("Missing properties")?
            .try_into()
            .context("Invalid properties")
    }
}

#[derive(Debug, Deserialize, Clone, Validate, Default)]
#[serde(default)]
pub struct UpdateConnectionRequest {
    #[validate(custom = "perroute_commons::types::name::validate")]
    name: Option<String>,

    #[validate(custom = "Properties::validate")]
    properties: Option<Value>,

    enabled: Option<bool>,
}

impl UpdateConnectionRequest {
    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn properties(&self) -> Result<Option<Properties>> {
        Ok(self.properties.clone().map(TryInto::try_into).transpose()?)
    }

    pub fn enabled(&self) -> Option<bool> {
        self.enabled
    }
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Eq)]
pub struct ConnectionResource {
    pub id: String,
    pub name: String,
    pub properties: Value,
    pub enabled: bool,
    pub plugin_id: String,
}

impl From<&Connection> for ConnectionResource {
    fn from(value: &Connection) -> Self {
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
            data: Some(self.into()),
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
