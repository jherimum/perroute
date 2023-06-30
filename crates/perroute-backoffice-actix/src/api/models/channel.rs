use crate::api::{
    response::{CollectionResource, Resource, SingleResource},
    Linkrelation, ResourceLink,
};
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

impl Resource for ChannelResource {}

impl From<Channel> for ChannelResource {
    fn from(value: Channel) -> Self {
        ChannelResource {
            code: value.code().to_owned(),
            name: value.name().to_owned(),
        }
    }
}

impl From<Channel> for SingleResource<ChannelResource> {
    fn from(value: Channel) -> Self {
        SingleResource::default()
            .with_data(value.clone().into())
            .with_link(
                Linkrelation::Self_,
                ResourceLink::Channel(value.code().clone()),
            )
            .with_link(Linkrelation::Channels, ResourceLink::Channels)
    }
}

impl From<Vec<Channel>> for CollectionResource<ChannelResource> {
    fn from(value: Vec<Channel>) -> Self {
        CollectionResource::default()
            .with_link(Linkrelation::Self_, ResourceLink::Channels)
            .with_resources(value.into_iter().map(Channel::into).collect())
    }
}
