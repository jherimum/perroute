use serde::{Deserialize, Serialize};

use crate::storage::database_models::channel::Channel;

#[derive(Clone, Serialize)]
pub struct ChannelResource {
    id: uuid::Uuid,
    code: String,
    description: Option<String>,
}

impl From<Channel> for ChannelResource {
    fn from(value: Channel) -> Self {
        ChannelResource {
            id: value.id,
            code: value.code,
            description: value.description,
        }
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct CreateChannelRequest {
    pub code: String,
    pub description: Option<String>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct UpdateChannelRequest {
    pub description: Option<String>,
}
