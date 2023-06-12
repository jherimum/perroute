use perroute_commons::types::{code::Code, id::Id};
use perroute_storage::models::channel::Channel;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
pub struct ChannelResource {
    id: Id,
    code: Code,
    name: String,
}

impl From<Channel> for ChannelResource {
    fn from(value: Channel) -> Self {
        ChannelResource {
            id: *value.id(),
            code: value.code().to_owned(),
            name: value.name().to_owned(),
        }
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct CreateChannelRequest {
    pub code: Code,
    pub name: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct UpdateChannelRequest {
    pub name: String,
}
