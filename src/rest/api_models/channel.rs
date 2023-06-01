use serde::{Deserialize, Serialize};

use crate::storage::database_models::channel::Channel;

#[derive(Clone, Serialize)]
pub struct ChannelResource {
    id: uuid::Uuid,
    code: String,
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
    pub code: String,
    pub name: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct UpdateChannelRequest {
    pub name: String,
}
