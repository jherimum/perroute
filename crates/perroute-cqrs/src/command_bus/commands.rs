use derive_builder::Builder;
use derive_getters::Getters;
use derive_new::new;
use perroute_commons::types::{actor::Actor, code::Code, id::Id};
use perroute_storage::models::command_log::CommandLog;
use serde::Serialize;
use std::fmt::Debug;
use strum_macros::Display;

pub trait Command: Debug + Serialize + Clone + PartialEq + Eq + Send + Sync {
    fn ty(&self) -> CommandType;

    fn to_log<E>(&self, actor: &Actor, error: Option<&E>) -> CommandLog
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        CommandLog::new(self.ty(), serde_json::to_value(self).unwrap(), actor, error)
    }
}

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

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct CreateChannelCommand {
    channel_id: Id,
    code: Code,
    name: String,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct DeleteChannelCommand {
    channel_id: Id,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct UpdateChannelCommand {
    channel_id: Id,
    name: String,
}

impl_command!(self::CreateChannelCommand, CommandType::CreateChannel);
impl_command!(self::DeleteChannelCommand, CommandType::DeleteChannel);
impl_command!(self::UpdateChannelCommand, CommandType::UpdateChannel);
