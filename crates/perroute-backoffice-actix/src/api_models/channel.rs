use derive_getters::Getters;
use perroute_commons::types::code::Code;
use perroute_storage::models::channel::Channel;
use serde::Serialize;

use crate::api::{ApiResource, Linkrelation, ResourceLink};

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

impl From<&Channel> for ChannelResource {
    fn from(value: &Channel) -> Self {
        ChannelResource {
            code: value.code().to_owned(),
            name: value.name().to_owned(),
        }
    }
}

impl From<Channel> for ApiResource<ChannelResource> {
    fn from(value: Channel) -> Self {
        ApiResource::<ChannelResource>::default()
            .with_data(ChannelResource::from(&value))
            .with_link(
                Linkrelation::Self_,
                ResourceLink::Channel(value.code().clone()),
            )
            .with_link(Linkrelation::Channels, ResourceLink::Channels)
    }
}
