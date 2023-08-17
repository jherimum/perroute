use crate::{
    api::response::{CollectionResourceModel, Links, ResourceBuilder, SingleResourceModel},
    links::{Linkrelation, ResourceLink},
};
use actix_web::HttpRequest;
use derive_getters::Getters;
use perroute_commons::types::{id::Id, priority::Priority, properties::Properties};
use perroute_connectors::types::DispatchType;
use perroute_storage::models::channel::Channel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Getters)]
pub struct CreateChannelRequest {
    business_id: Id,
    connection_id: Id,
    dispatch_type: DispatchType,
    properties: Properties,
    priority: Priority,
}

#[derive(Debug, Deserialize, Clone, Getters)]
pub struct UpdateChannelRequest {
    properties: Properties,
    priority: Priority,
    enabled: bool,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Eq)]
pub struct ChannelResource {
    id: Id,
    dispatch_type: DispatchType,
    properties: Properties,
    enabled: bool,
    priority: Priority,
}

impl From<Channel> for ChannelResource {
    fn from(value: Channel) -> Self {
        Self {
            id: *value.id(),
            dispatch_type: *value.dispatch_type(),
            properties: value.properties().clone(),
            enabled: *value.enabled(),
            priority: *value.priority(),
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
