use crate::{
    api::response::AsUrl,
    routes::{
        business_unit::BusinessUnitRouter, channel::ChannelsRouter, connection::ConnectionsRouter,
        message_type::MessageTypeRouter, plugin::PluginRouter, route::RouteRouter,
        schema::SchemaRouter, template::TemplateRouter,
    },
};
use actix_web::HttpRequest;
use perroute_commons::types::id::Id;
use perroute_connectors::types::plugin_id::ConnectorPluginId;
use serde::Serialize;
use std::string::ToString;
use tap::TapFallible;
use url::Url;

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Clone, Copy, strum_macros::Display)]
#[strum(serialize_all = "snake_case")]
pub enum Linkrelation {
    #[serde(rename = "self")]
    Self_,
    #[serde(rename = "business_units")]
    BusinessUnits,
    #[serde(rename = "business_unit")]
    BusinessUnit,
    #[serde(rename = "message_types")]
    MessageTypes,
    #[serde(rename = "routes")]
    Routes,

    #[serde(rename = "schemas")]
    Schemas,

    #[serde(rename = "schema")]
    Schema,

    #[serde(rename = "connections")]
    Connections,
    #[serde(rename = "connection")]
    Connection,

    #[serde(rename = "plugin")]
    Plugin,

    #[serde(rename = "plugins")]
    Plugins,

    #[serde(rename = "channel")]
    Channel,

    #[serde(rename = "channels")]
    Channels,
}

#[derive(Debug, Serialize, Clone)]
pub enum ResourceLink {
    BusinessUnit(Id),
    BusinessUnits,

    MessageTypes,
    MessageType(Id),

    Schemas(Id),
    Schema(Id, Id),

    Templates,
    Template(Id),

    Routes,
    Route(Id),

    Connections,
    Connection(Id),

    Plugin(ConnectorPluginId),
    Plugins,

    Channel(Id),
    Channels,
}

impl AsUrl for ResourceLink {
    fn as_url(&self, req: &HttpRequest) -> Url {
        match self {
            Self::BusinessUnit(id) => {
                req.url_for(BusinessUnitRouter::BU_RESOURCE_NAME, [id.to_string()])
            }
            Self::BusinessUnits => req.url_for_static(BusinessUnitRouter::BUS_RESOURCE_NAME),

            Self::MessageTypes => {
                req.url_for_static(MessageTypeRouter::MESSAGE_TYPES_RESOURCE_NAME)
            }
            Self::MessageType(message_type_id) => req.url_for(
                MessageTypeRouter::MESSAGE_TYPE_RESOURCE_NAME,
                [message_type_id.to_string()],
            ),

            Self::Schemas(message_type_id) => req.url_for(
                SchemaRouter::SCHEMAS_RESOURCE_NAME,
                [message_type_id.to_string()],
            ),
            Self::Schema(message_type_id, schema_id) => req.url_for(
                SchemaRouter::SCHEMA_RESOURCE_NAME,
                [message_type_id.to_string(), schema_id.to_string()],
            ),

            Self::Templates => req.url_for_static(TemplateRouter::TEMPLATES_RESOURCE_NAME),

            Self::Template(template_id) => req.url_for(
                TemplateRouter::TEMPLATE_RESOURCE_NAME,
                [template_id.to_string()],
            ),

            Self::Routes => req.url_for_static(RouteRouter::ROUTES_RESOURCE_NAME),
            Self::Route(route_id) => {
                req.url_for(RouteRouter::ROUTE_RESOURCE_NAME, [route_id.to_string()])
            }
            Self::Connections => req.url_for_static(ConnectionsRouter::CONNS_RESOURCE_NAME),
            Self::Connection(connection_id) => req.url_for(
                ConnectionsRouter::CONN_RESOURCE_NAME,
                [connection_id.to_string()],
            ),

            Self::Plugin(plugin_id) => {
                req.url_for(PluginRouter::PLUGIN_RESOURCE_NAME, [plugin_id.to_string()])
            }
            Self::Plugins => req.url_for_static(PluginRouter::PLUGINS_RESOURCE_NAME),

            Self::Channels => req.url_for_static(ChannelsRouter::CHANNELS_RESOURCE_NAME),
            Self::Channel(id) => {
                req.url_for(ChannelsRouter::CHANNEL_RESOURCE_NAME, [id.to_string()])
            }
        }
        .tap_err(|e| tracing::error!("Failed to build url: {}", e))
        .expect("msg")
    }
}

impl ResourceLink {
    pub fn as_location_header(&self, req: &HttpRequest) -> (String, String) {
        (
            actix_web::http::header::LOCATION.as_str().to_string(),
            self.as_url(req).to_string(),
        )
    }
}
