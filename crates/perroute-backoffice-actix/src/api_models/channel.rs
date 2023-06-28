use perroute_commons::types::{code::Code, id::Id};
use perroute_storage::models::channel::Channel;
use serde::Serialize;

use crate::api::{ApiResource, Linkrelation, ResourceLink};

#[derive(Debug, serde::Deserialize, Clone)]
pub struct CreateChannelRequest {
    pub code: Code,
    pub name: String,
}

#[derive(Clone, Serialize, Debug)]
pub struct ChannelResource {
    id: Id,
    code: Code,
    name: String,
}

impl From<&Channel> for ChannelResource {
    fn from(value: &Channel) -> Self {
        ChannelResource {
            id: *value.id(),
            code: value.code().to_owned(),
            name: value.name().to_owned(),
        }
    }
}

impl From<Channel> for ApiResource<ChannelResource> {
    fn from(value: Channel) -> Self {
        ApiResource::<ChannelResource>::default()
            .with_data(ChannelResource::from(&value))
            .with_link(Linkrelation::Self_, ResourceLink::Channel(*value.id()))
            .with_link(Linkrelation::Channels, ResourceLink::Channels)
    }
}
