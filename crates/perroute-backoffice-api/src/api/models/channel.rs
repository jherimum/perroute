use crate::{
    api::response::{CollectionResourceModel, Links, ResourceBuilder, SingleResourceModel},
    links::{Linkrelation, ResourceLink},
};
use actix_web::HttpRequest;
use perroute_storage::models::channel::Channel;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;

#[derive(Debug, Deserialize, Clone, Validate, Default)]
#[serde(default)]
pub struct CreateChannelRequest {
    #[validate(required)]
    #[validate(custom = "perroute_commons::types::id::Id::validate")]
    pub business_id: Option<String>,

    #[validate(required)]
    #[validate(custom = "perroute_commons::types::id::Id::validate")]
    pub connection_id: Option<String>,

    #[validate(required)]
    #[validate(custom = "perroute_connectors::types::dispatch_type::DispatchType::validate")]
    pub dispatch_type: Option<String>,

    #[validate(required)]
    #[validate(custom = "perroute_commons::types::properties::Properties::validate")]
    pub properties: Option<Value>,

    #[validate(required)]
    #[validate(custom = "perroute_commons::types::priority::Priority::validate")]
    pub priority: Option<i32>,
}

#[derive(Debug, Deserialize, Clone, Validate, Default)]
#[serde(default)]
pub struct UpdateChannelRequest {
    #[validate(required)]
    #[validate(custom = "perroute_commons::types::properties::Properties::validate")]
    pub properties: Option<Value>,

    #[validate(custom = "perroute_commons::types::priority::Priority::validate")]
    pub priority: Option<i32>,

    pub enabled: Option<bool>,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Eq)]
pub struct ChannelResource {
    id: String,
    dispatch_type: String,
    properties: Value,
    enabled: bool,
    priority: i32,
}

impl From<Channel> for ChannelResource {
    fn from(value: Channel) -> Self {
        Self {
            id: value.id().into(),
            dispatch_type: value.dispatch_type().into(),
            properties: value.properties().into(),
            enabled: *value.enabled(),
            priority: value.priority().into(),
        }
    }
}

impl ResourceBuilder<SingleResourceModel<ChannelResource>> for Channel {
    fn build(&self, req: &HttpRequest) -> SingleResourceModel<ChannelResource> {
        SingleResourceModel {
            data: Some(ChannelResource::from(self.clone())),
            links: Links::default()
                .add(Linkrelation::Self_, ResourceLink::Channel(*self.id()))
                .add(Linkrelation::Channels, ResourceLink::Channels)
                .add(
                    Linkrelation::Connection,
                    ResourceLink::Connection(*self.connection_id()),
                )
                .add(
                    Linkrelation::BusinessUnit,
                    ResourceLink::BusinessUnit(*self.business_unit_id()),
                )
                .as_url_map(req),
        }
    }
}

impl ResourceBuilder<CollectionResourceModel<ChannelResource>> for Vec<Channel> {
    fn build(&self, req: &HttpRequest) -> CollectionResourceModel<ChannelResource> {
        CollectionResourceModel {
            data: self.iter().map(|c| c.build(req)).collect(),
            links: Links::default()
                .add(Linkrelation::Self_, ResourceLink::Channels)
                .as_url_map(req),
        }
    }
}
