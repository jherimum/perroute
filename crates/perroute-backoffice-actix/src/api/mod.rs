use actix_web::HttpRequest;
use perroute_commons::types::code::Code;
use serde::Serialize;
use tap::TapFallible;
use url::Url;

use crate::routes::channel::{CHANNELS_RESOUCE_LINK, CHANNEL_RESOUCE_LINK};

pub mod models;
pub mod response;

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Clone, Copy)]
pub enum Linkrelation {
    Self_,
    Channels,
}

#[derive(Debug, Serialize, Clone)]
pub enum ResourceLink {
    Channel(Code),
    Channels,
}

impl ResourceLink {
    pub fn as_url(&self, req: &HttpRequest) -> Url {
        match self {
            ResourceLink::Channel(id) => req.url_for(CHANNEL_RESOUCE_LINK, [id.to_string()]),
            ResourceLink::Channels => req.url_for_static(CHANNELS_RESOUCE_LINK),
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
