use std::collections::HashSet;

use crate::{
    command_bus::{
        bus::CommandBusContext, commands::CommandType, error::CommandBusError,
        handlers::CommandHandler,
    },
    impl_command, into_event,
};
use chrono::NaiveDateTime;
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{
    actor::Actor, code::Code, dispatch_type::DispatchType, id::Id, payload::Payload,
    recipient::Recipient,
};
use perroute_messaging::events::EventType;
use perroute_storage::{
    models::{
        channel::{Channel, ChannelsQueryBuilder},
        message::{Message, MessageBuilder, Status},
        schema::Version,
    },
    query::FetchableModel,
};
use serde::Serialize;
use sqlx::{types::Json, PgPool};
use thiserror::Error;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct CreateMessageCommand {
    #[builder(default)]
    message_id: Id,

    payload: Payload,
    recipient: Recipient,

    #[builder(default)]
    scheduled_to: Option<NaiveDateTime>,

    channel_code: Code,
    message_type_code: Code,
    schema_version: Version,

    #[builder(default)]
    include_dispatcher_types: HashSet<DispatchType>,

    #[builder(default)]
    exclude_dispatcher_types: HashSet<DispatchType>,
}

impl_command!(CreateMessageCommand, CommandType::CreateMessage);
into_event!(
    CreateMessageCommand,
    EventType::MessageCreated,
    |cmd: &CreateMessageCommand| { cmd.message_id }
);

#[derive(Error, Debug)]
pub enum CreateMessageCommandError {
    #[error("channel not found: {0}")]
    ChannelNotFound(Code),
    #[error("Message type not found: {0}")]
    MessageTypeNotFound(Code),

    #[error("Schema not found: {0}")]
    SchemaNotFound(Version),

    #[error("Channel {0} is disabled")]
    ChannelDisabled(Code),

    #[error("Message type {0} is disabled")]
    MessageTypeDisabled(Code),

    #[error("Schema {0} is disabled")]
    SchemaDisabled(Version),

    #[error("Schema {0} is not published")]
    SchemaBotPublished(Version),
}

#[derive(Debug)]
pub struct CreateMessageCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for CreateMessageCommandHandler {
    type Command = CreateMessageCommand;
    type Output = Message;

    #[tracing::instrument(name = "create_message_handler", skip(self, ctx))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        actor: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        let channel = retrieve_channel(ctx.pool(), cmd.channel_code()).await?;
        if !channel.enabled() {
            return Err(CreateMessageCommandError::ChannelDisabled(channel.code().clone()).into());
        }

        let message_type = channel
            .message_type_by_code(ctx.pool(), cmd.message_type_code().clone())
            .await?
            .ok_or_else(|| {
                CreateMessageCommandError::MessageTypeNotFound(cmd.message_type_code().clone())
            })?;

        if !message_type.enabled() {
            return Err(CreateMessageCommandError::MessageTypeDisabled(
                message_type.code().clone(),
            )
            .into());
        }

        let schema = message_type
            .schema_by_version(ctx.pool(), *cmd.schema_version())
            .await?
            .ok_or_else(|| CreateMessageCommandError::SchemaNotFound(*cmd.schema_version()))?;

        if !schema.enabled() {
            return Err(CreateMessageCommandError::SchemaDisabled(*cmd.schema_version()).into());
        }

        if !schema.published() {
            return Err(CreateMessageCommandError::SchemaDisabled(*cmd.schema_version()).into());
        }

        schema.schema().validate(cmd.payload()).unwrap();

        MessageBuilder::default()
            .id(*cmd.message_id())
            .status(Status::Pending)
            .payload(cmd.payload().clone())
            .scheduled_to(*cmd.scheduled_to())
            .schema_id(*schema.id())
            .message_type_id(*schema.message_type_id())
            .channel_id(*channel.id())
            .include_dispatcher_types(Json(cmd.include_dispatcher_types().clone()))
            .exclude_dispatcher_types(Json(cmd.exclude_dispatcher_types().clone()))
            .recipient(Json(cmd.recipient().clone()))
            .build()
            .unwrap()
            .save(ctx.tx())
            .await
            .map_err(CommandBusError::from)
    }
}

async fn retrieve_channel(pool: &PgPool, code: &Code) -> Result<Channel, CommandBusError> {
    Channel::find(
        pool,
        ChannelsQueryBuilder::default()
            .code(Some(code.clone()))
            .build()
            .unwrap(),
    )
    .await?
    .ok_or_else(|| CreateMessageCommandError::ChannelNotFound(code.clone()).into())
}
