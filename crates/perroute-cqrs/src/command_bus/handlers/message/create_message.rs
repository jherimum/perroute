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
    actor::Actor, code::Code, id::Id, payload::Payload, recipient::Recipient, version::Version,
};
use perroute_connectors::types::DispatchTypes;
use perroute_messaging::events::EventType;
use perroute_storage::{
    models::{
        business_unit::{BusinessUnit, BusinessUnitQueryBuilder},
        message::{Message, MessageBuilder, Status},
        message_type::{MessageType, MessageTypeQueryBuilder},
        schema::{Schema, SchemasQueryBuilder},
    },
    query::FetchableModel,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct CreateMessageCommand {
    #[builder(default)]
    id: Id,

    payload: Payload,
    recipient: Recipient,

    bu_code: Code,
    message_type_code: Code,
    schema_version: Version,

    #[builder(default)]
    dispatcher_types: DispatchTypes,
}

impl_command!(CreateMessageCommand, CommandType::CreateMessage);
into_event!(
    CreateMessageCommand,
    EventType::MessageCreated,
    |cmd: &CreateMessageCommand| { cmd.id }
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
        let bu = BusinessUnit::find(
            ctx.pool(),
            BusinessUnitQueryBuilder::default().build().unwrap(),
        )
        .await?
        .unwrap();

        let message_type = MessageType::find(
            ctx.pool(),
            MessageTypeQueryBuilder::default()
                .business_unit_id(Some(*bu.id()))
                .build()
                .unwrap(),
        )
        .await?
        .unwrap();

        let schema = Schema::find(
            ctx.pool(),
            SchemasQueryBuilder::default()
                .message_type_id(Some(*message_type.id()))
                .version(Some(cmd.schema_version))
                .build()
                .unwrap(),
        )
        .await
        .unwrap()
        .unwrap();

        if !schema.enabled() {
            return Err(CreateMessageCommandError::SchemaDisabled(cmd.schema_version).into());
        }

        if !schema.published() {
            return Err(CreateMessageCommandError::SchemaDisabled(cmd.schema_version).into());
        }

        let message_type = schema.message_type(ctx.pool()).await?;

        if !message_type.enabled() {
            return Err(CreateMessageCommandError::MessageTypeDisabled(
                message_type.code().clone(),
            )
            .into());
        }

        schema.value().validate(&cmd.payload).unwrap();

        MessageBuilder::default()
            .id(cmd.id)
            .status(Status::Pending)
            .payload(cmd.payload)
            .schema_id(*schema.id())
            .message_type_id(*schema.message_type_id())
            .business_unit_id(*schema.business_unit_id())
            .dispatcher_types(cmd.dispatcher_types)
            .recipient(cmd.recipient)
            .build()
            .unwrap()
            .save(ctx.tx())
            .await
            .map_err(CommandBusError::from)
    }
}
