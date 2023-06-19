use perroute_commons::types::id::Id;
use strum_macros::Display;

pub mod channel;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    ChannelEvent(ChannelEvent),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChannelEvent {
    Created(Id),
    Updated(Id),
    Deleted(Id),
}

#[derive(Display, Debug, Clone)]
pub enum CommandType {
    CreateChannel,
    UpdateChannel,
    DeleteChannel,
}

impl From<CommandType> for String {
    fn from(value: CommandType) -> Self {
        value.to_string()
    }
}
