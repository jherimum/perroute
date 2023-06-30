use actix_web::HttpRequest;
use perroute_commons::types::id::Id;
use serde::Serialize;
use tap::TapFallible;
use url::Url;

use crate::routes::{
    channel::{CHANNELS_RESOURCE_NAME, CHANNEL_RESOURCE_NAME},
    message_type::{MESSAGE_TYPES_RESOURCE_NAME, MESSAGE_TYPE_RESOURCE_NAME},
};

pub mod models;
pub mod response;

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Clone, Copy)]
pub enum Linkrelation {
    Self_,
    Channels,
    Channel,
    MessageTypes,
}

#[derive(Debug, Serialize, Clone)]
pub enum ResourceLink {
    Channel(Id),
    Channels,
    MessageType(Id, Id),
    MessageTypes(Id),
}

impl ResourceLink {
    pub fn as_url(&self, req: &HttpRequest) -> Url {
        match self {
            ResourceLink::Channel(id) => req.url_for(CHANNEL_RESOURCE_NAME, [id.to_string()]),
            ResourceLink::Channels => req.url_for_static(CHANNELS_RESOURCE_NAME),
            ResourceLink::MessageTypes(channel_code) => {
                req.url_for(MESSAGE_TYPES_RESOURCE_NAME, [channel_code.to_string()])
            }
            ResourceLink::MessageType(channel_id, message_type_id) => req.url_for(
                MESSAGE_TYPE_RESOURCE_NAME,
                [channel_id.to_string(), message_type_id.to_string()],
            ),
        }
        .tap_err(|e| tracing::error!("Failed to build url: {}", e))
        .expect("msg")
    }
    pub fn as_location_header(&self, req: &HttpRequest) -> (String, String) {
        (
            actix_web::http::header::LOCATION.as_str().to_string(),
            self.as_url(req).to_string(),
        )
    }
}
