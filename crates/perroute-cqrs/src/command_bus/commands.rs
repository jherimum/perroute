use chrono::{NaiveDateTime, Utc};
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::{
    new_id,
    types::{
        actor::Actor, code::Code, id::Id, json_schema::JsonSchema, payload::Payload,
        template::TemplateSnippet,
    },
};
use perroute_storage::models::command_log::{CommandLog, CommandLogBuilder};
use serde::Serialize;
use std::fmt::Debug;
use strum_macros::Display;

use crate::{command, impl_command};

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

    CreateTemplate,
    UpdateTemplate,
    DeleteTemplate,

    CreateMessage,
}

impl From<CommandType> for String {
    fn from(value: CommandType) -> Self {
        value.to_string()
    }
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct CreateChannelCommand {
    #[builder(default)]
    channel_id: Id,
    code: Code,
    name: String,
}

impl_command!(CreateChannelCommand, CommandType::CreateChannel);

command!(
    DeleteChannelCommand,
    CommandType::DeleteChannel,
    channel_id: Id
);
command!(
    UpdateChannelCommand,
    CommandType::UpdateChannel,
    channel_id: Id,
    name: String
);

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct CreateMessageTypeCommand {
    #[builder(default)]
    message_type_id: Id,
    code: Code,
    description: String,
    channel_id: Id,
}

impl_command!(CreateMessageTypeCommand, CommandType::CreateMessageType);

command!(
    UpdateMessageTypeCommand,
    CommandType::UpdateMessageType,
    message_type_id: Id,
    description: String,
    enabled: bool
);

command!(
    DeleteMessageTypeCommand,
    CommandType::DeleteMessageType,
    message_type_id: Id
);

command!(
    CreateSchemaCommand,
    CommandType::CreateSchema,
    schema_id: Id,
    message_type_id: Id,
    schema: JsonSchema
);

command!(
    UpdateSchemaCommand,
    CommandType::UpdateSchema,
    schema_id: Id,
    schema: JsonSchema
);

command!(
    DeleteSchemaCommand,
    CommandType::DeleteSchema,
    schema_id: Id
);

command!(
    PublishSchemaCommand,
    CommandType::PublishSchema,
    schema_id: Id
);

//templates

command!(
    CreateTemplateCommand,
    CommandType::CreateTemplate,
    template_id: Id,
    schema_id: Id,
    name: String,
    html: Option<TemplateSnippet>,
    text: Option<TemplateSnippet>,
    subject: Option<TemplateSnippet>
);

command!(
    UpdateTemplateCommand,
    CommandType::UpdateTemplate,
    template_id: Id,
    name: String,
    html: Option<TemplateSnippet>,
    text: Option<TemplateSnippet>,
    subject: Option<TemplateSnippet>
);

command!(
    DeleteTemplateCommand,
    CommandType::DeleteTemplate,
    template_id: Id
);

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct CreateMessageCommand {
    #[builder(default)]
    message_id: Id,
    payload: Payload,
    #[builder(default)]
    scheduled_to: Option<NaiveDateTime>,
    schema_id: Id,
}
impl_command!(CreateMessageCommand, CommandType::CreateMessage);
