use perroute_commons::types::code::Code;
use perroute_storage::models::channel::Channel;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
pub struct ChannelResource {
    id: uuid::Uuid,
    code: Code,
    name: String,
}

impl From<Channel> for ChannelResource {
    fn from(value: Channel) -> Self {
        ChannelResource {
            id: value.id,
            code: value.code,
            name: value.name,
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
