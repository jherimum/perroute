use chrono::Utc;
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::{
    new_id,
    types::{actor::Actor, code::Code, id::Id},
};
use perroute_storage::models::command_log::{CommandLog, CommandLogBuilder};
use serde::Serialize;
use serde_json::Value;
use std::fmt::Debug;
use strum_macros::Display;

macro_rules! impl_command {
    ($cmd: ty, $ty: expr) => {
        impl Command for $cmd {
            fn ty(&self) -> CommandType {
                $ty
            }
        }
    };
}

pub trait Command: Debug + Serialize + Clone + PartialEq + Eq + Send + Sync {
    fn ty(&self) -> CommandType;

    fn to_log<E>(&self, actor: &Actor, error: Option<&E>) -> CommandLog
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        CommandLogBuilder::default()
            .id(new_id!())
            .actor_type(*actor.ty())
            .actor_id(*actor.id())
            .command_type(self.ty())
            .payload(serde_json::to_value(self).unwrap())
            .created_at(Utc::now().naive_utc())
            .error(error.map(|e| format!("{e}")))
            .build()
            .unwrap()
    }
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

    CreateMessageType,
    UpdateMessageType,
    DeleteMessageType,

    CreateSchema,
    UpdateSchema,
    DeleteSchema,
    PublishSchema,
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

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct CreateMessageTypeCommand {
    message_type_id: Id,
    code: Code,
    description: String,
    channel_id: Id,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct UpdateMessageTypeCommand {
    message_type_id: Id,
    description: String,
    enabled: bool,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct DeleteMessageTypeCommand {
    message_type_id: Id,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct CreateSchemaCommand {
    schema_id: Id,
    schema: Value,
    message_type_id: Id,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct UpdateSchemaCommand {
    schema_id: Id,
    schema: serde_json::Value,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct DeleteSchemaCommand {
    schema_id: Id,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct PublishSchemaCommand {
    schema_id: Id,
}

impl_command!(CreateChannelCommand, CommandType::CreateChannel);
impl_command!(DeleteChannelCommand, CommandType::DeleteChannel);
impl_command!(UpdateChannelCommand, CommandType::UpdateChannel);
impl_command!(CreateMessageTypeCommand, CommandType::CreateMessageType);
impl_command!(UpdateMessageTypeCommand, CommandType::UpdateMessageType);
impl_command!(DeleteMessageTypeCommand, CommandType::DeleteMessageType);
impl_command!(CreateSchemaCommand, CommandType::CreateSchema);
impl_command!(UpdateSchemaCommand, CommandType::UpdateSchema);
impl_command!(DeleteSchemaCommand, CommandType::DeleteSchema);
impl_command!(PublishSchemaCommand, CommandType::PublishSchema);
