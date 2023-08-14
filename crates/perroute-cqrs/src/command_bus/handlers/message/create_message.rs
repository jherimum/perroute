use std::collections::HashSet;

use crate::{
    command_bus::{
        bus::CommandBusContext, commands::CommandType, error::CommandBusError,
        handlers::CommandHandler,
    },
    impl_command, into_event,
};
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{
    actor::Actor, code::Code, id::Id, payload::Payload, recipient::Recipient,
};
use perroute_connectors::api::DispatchType;
use perroute_messaging::events::EventType;
use perroute_storage::{
    models::{
        business_unit::{BusinessUnit, BusinessUnitQueryBuilder},
        message::{Message, MessageBuilder, Status},
        message_type::{MessageType, MessageTypeQueryBuilder},
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

    bu_code: Code,
    message_type_code: Code,
    schema_version: Version,

    #[builder(default)]
    dispatcher_types: HashSet<DispatchType>,
}

impl_command!(CreateMessageCommand, CommandType::CreateMessage);
into_event!(
    CreateMessageCommand,
    EventType::MessageCreated,
    |cmd: &CreateMessageCommand| { cmd.message_id }
);

#[derive(Error, Debug)]
pub enum CreateMessageCommandError {
    #[error("Business unit not found: {0}")]
    BusinessUnitNotFound(Code),
    #[error("Message type not found: {0}")]
    MessageTypeNotFound(Code),

    #[error("Schema not found: {0}")]
    SchemaNotFound(Version),

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
        let bu = retrieve_bu(ctx.pool(), cmd.bu_code()).await?;

        let message_type = MessageType::find(
            ctx.pool(),
            MessageTypeQueryBuilder::default()
                .code(Some(cmd.message_type_code().clone()))
                .build()
                .unwrap(),
        )
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

        schema.value().validate(cmd.payload()).unwrap();

        MessageBuilder::default()
            .id(*cmd.message_id())
            .status(Status::Pending)
            .payload(cmd.payload().clone())
            .schema_id(*schema.id())
            .message_type_id(*schema.message_type_id())
            .bu_id(*bu.id())
            .dispatcher_types(Json(cmd.dispatcher_types().clone()))
            .recipient(Json(cmd.recipient().clone()))
            .build()
            .unwrap()
            .save(ctx.tx())
            .await
            .map_err(CommandBusError::from)
    }
}

async fn retrieve_bu(pool: &PgPool, code: &Code) -> Result<BusinessUnit, CommandBusError> {
    BusinessUnit::find(
        pool,
        BusinessUnitQueryBuilder::default()
            .code(Some(code.clone()))
            .build()
            .unwrap(),
    )
    .await?
    .ok_or_else(|| CreateMessageCommandError::BusinessUnitNotFound(code.clone()).into())
}
