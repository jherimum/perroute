use crate::{
    api::response::AsUrl,
    routes::{
        api_key::ApiKeyRouter, channel::ChannelRouter, message_type::MessageTypeRouter,
        route::RouteRouter, schema::SchemaRouter, template::TemplateRouter,
    },
};
use actix_web::HttpRequest;
use perroute_commons::types::id::Id;
use serde::Serialize;
use tap::TapFallible;
use url::Url;

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Clone, Copy, strum_macros::Display)]
#[strum(serialize_all = "snake_case")]
pub enum Linkrelation {
    #[serde(rename = "self")]
    Self_,
    #[serde(rename = "channels")]
    Channels,
    #[serde(rename = "channel")]
    Channel,
    #[serde(rename = "message_types")]
    MessageTypes,
    #[serde(rename = "routes")]
    Routes,
    #[serde(rename = "schemas")]
    Schemas,
}

#[derive(Debug, Serialize, Clone)]
pub enum ResourceLink {
    Channel(Id),
    Channels,

    MessageTypes(Id),
    MessageType(Id, Id),

    Schemas(Id, Id),
    Schema(Id, Id, Id),

    Templates(Id, Id, Id),
    Template(Id, Id, Id, Id),

    Routes(Id),
    Route(Id, Id),

    ApiKeys,
    ApiKey(Id),
}

impl AsUrl for ResourceLink {
    fn as_url(&self, req: &HttpRequest) -> Url {
        match self {
            ResourceLink::Channel(id) => {
                req.url_for(ChannelRouter::CHANNEL_RESOURCE_NAME, [id.to_string()])
            }
            ResourceLink::Channels => req.url_for_static(ChannelRouter::CHANNELS_RESOURCE_NAME),

            ResourceLink::MessageTypes(channel_id) => req.url_for(
                MessageTypeRouter::MESSAGE_TYPES_RESOURCE_NAME,
                [channel_id.to_string()],
            ),
            ResourceLink::MessageType(channel_id, message_type_id) => req.url_for(
                MessageTypeRouter::MESSAGE_TYPE_RESOURCE_NAME,
                [channel_id.to_string(), message_type_id.to_string()],
            ),

            ResourceLink::Schemas(channel_id, message_type_id) => req.url_for(
                SchemaRouter::SCHEMAS_RESOURCE_NAME,
                [channel_id.to_string(), message_type_id.to_string()],
            ),
            ResourceLink::Schema(channel_id, message_type_id, schema_id) => req.url_for(
                SchemaRouter::SCHEMA_RESOURCE_NAME,
                [
                    channel_id.to_string(),
                    message_type_id.to_string(),
                    schema_id.to_string(),
                ],
            ),

            ResourceLink::Templates(channel_id, message_type_id, schema_id) => req.url_for(
                TemplateRouter::TEMPLATES_RESOURCE_NAME,
                [
                    channel_id.to_string(),
                    message_type_id.to_string(),
                    schema_id.to_string(),
                ],
            ),

            ResourceLink::Template(channel_id, message_type_id, schema_id, template_id) => req
                .url_for(
                    TemplateRouter::TEMPLATE_RESOURCE_NAME,
                    [
                        channel_id.to_string(),
                        message_type_id.to_string(),
                        schema_id.to_string(),
                        template_id.to_string(),
                    ],
                ),

            ResourceLink::Routes(channel_id) => {
                req.url_for(RouteRouter::ROUTES_RESOURCE_NAME, [channel_id.to_string()])
            }
            ResourceLink::Route(channel_id, route_id) => req.url_for(
                RouteRouter::ROUTE_RESOURCE_NAME,
                [channel_id.to_string(), route_id.to_string()],
            ),

            ResourceLink::ApiKeys => req.url_for_static(ApiKeyRouter::API_KEY_RESOURCES_NAME),
            ResourceLink::ApiKey(id) => {
                req.url_for(ApiKeyRouter::API_KEY_RESOURCE_NAME, [id.to_string()])
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
