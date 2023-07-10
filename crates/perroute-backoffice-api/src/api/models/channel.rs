use crate::{
    api::response::{Links, ResourceBuilder},
    links::{Linkrelation, ResourceLink},
    routes::prelude::ResourceModel,
};
use actix_web::HttpRequest;
use derive_getters::Getters;
use perroute_commons::{prelude::Id, types::code::Code};
use perroute_storage::models::channel::Channel;
use serde::Serialize;

#[derive(Debug, serde::Deserialize, Clone, Getters)]
pub struct CreateChannelRequest {
    code: Code,
    name: String,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct UpdateChannelRequest {
    pub name: String,
}

#[derive(Clone, Serialize, Debug)]
pub struct ChannelResource {
    id: Id,
    code: Code,
    name: String,
}

impl From<Channel> for ChannelResource {
    fn from(value: Channel) -> Self {
        ChannelResource {
            id: value.id().to_owned(),
            code: value.code().to_owned(),
            name: value.name().to_owned(),
        }
    }
}

impl ResourceBuilder<ResourceModel<ChannelResource>> for Channel {
    fn build(&self, req: &HttpRequest) -> ResourceModel<ChannelResource> {
        ResourceModel {
            data: Some(ChannelResource::from(self.clone())),
            links: Links::default()
                .add(Linkrelation::Self_, ResourceLink::Channel(*self.id()))
                .add(Linkrelation::Channels, ResourceLink::Channels)
                .add(
                    Linkrelation::MessageTypes,
                    ResourceLink::MessageTypes(*self.id()),
                )
                .add(Linkrelation::Routes, ResourceLink::Routes(*self.id()))
                .as_url_map(req),
        }
    }
}

impl ResourceBuilder<ResourceModel<Vec<ResourceModel<ChannelResource>>>> for Vec<Channel> {
    fn build(&self, req: &HttpRequest) -> ResourceModel<Vec<ResourceModel<ChannelResource>>> {
        ResourceModel {
            data: Some(self.iter().map(|c| c.build(req)).collect()),
            links: Links::default()
                .add(Linkrelation::Self_, ResourceLink::Channels)
                .as_url_map(req),
        }
    }
}
