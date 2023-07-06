use crate::api::{
    response::{CollectionResourceModel, Links, ResourceBuilder, SingleResourceModel},
    Linkrelation, ResourceLink,
};
use actix_web::HttpRequest;
use derive_getters::Getters;
use perroute_commons::types::code::Code;
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
    code: Code,
    name: String,
}

impl From<Channel> for ChannelResource {
    fn from(value: Channel) -> Self {
        ChannelResource {
            code: value.code().to_owned(),
            name: value.name().to_owned(),
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
