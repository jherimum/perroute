use crate::api::{CollectionResource, Linkrelation, Links, ResourceLink, SingleResource};
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

impl From<Channel> for SingleResource<ChannelResource> {
    fn from(value: Channel) -> Self {
        SingleResource {
            data: value.clone().into(),
            links: Links::default()
                .add(
                    Linkrelation::Self_,
                    crate::api::ResourceLink::Channel(value.code().clone()),
                )
                .add(Linkrelation::Channels, ResourceLink::Channels),
        }
    }
}

impl From<Vec<Channel>> for CollectionResource<ChannelResource> {
    fn from(value: Vec<Channel>) -> Self {
        CollectionResource {
            data: value.into_iter().map(Channel::into).collect(),
            links: Links::default().add(Linkrelation::Self_, ResourceLink::Channels),
        }
    }
}
