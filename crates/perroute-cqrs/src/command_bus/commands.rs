use perroute_commons::types::id::Id;
use strum_macros::Display;
pub mod channel;

#[macro_export]
macro_rules! impl_command {
    ($cmd: ty, $ty: expr) => {
        impl Command for $cmd {
            fn ty(&self) -> CommandType {
                $ty
            }
        }
    };
}

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
