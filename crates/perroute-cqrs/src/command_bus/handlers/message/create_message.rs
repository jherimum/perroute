use std::collections::HashSet;

use crate::{
    command_bus::{
        bus::CommandBusContext, commands::CommandType, handlers::CommandHandler, Result,
    },
    impl_command, into_event,
};
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{
    actor::Actor, code::Code, id::Id, json_schema::InvalidPayloadError, payload::Payload,
    version::Version,
};
use perroute_connectors::types::delivery::Delivery;
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
use sqlx::types::Json;
use tap::TapFallible;
use thiserror::Error;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct CreateMessageCommand {
    id: Id,
    payload: Payload,
    business_unit_code: Code,
    message_type_code: Code,
    schema_version: Version,
    deliveries: HashSet<Delivery>,
}

impl_command!(CreateMessageCommand, CommandType::CreateMessage);
into_event!(
    CreateMessageCommand,
    EventType::MessageCreated,
    |cmd: &CreateMessageCommand| { cmd.id }
);

#[derive(Error, Debug)]
pub enum CreateMessageError {
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
    SchemaNotPublished(Version),

    #[error(transparent)]
    InvalidPayload(#[from] InvalidPayloadError),
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
    ) -> Result<Self::Output> {
        let bu = BusinessUnit::find(
            ctx.pool(),
            BusinessUnitQueryBuilder::default()
                .code(Some(cmd.business_unit_code.clone()))
                .build()
                .unwrap(),
        )
        .await
        .tap_err(|e| tracing::error!("Failed to retrieve business unit:{e}"))?
        .ok_or_else(|| CreateMessageError::BusinessUnitNotFound(cmd.business_unit_code))?;

        let message_type = MessageType::find(
            ctx.pool(),
            MessageTypeQueryBuilder::default()
                .business_unit_id(Some(*bu.id()))
                .code(Some(cmd.message_type_code.clone()))
                .build()
                .unwrap(),
        )
        .await
        .tap_err(|e| tracing::error!("Failed to retrieve message type:{e}"))?
        .ok_or_else(|| CreateMessageError::MessageTypeNotFound(cmd.message_type_code))?;

        if !message_type.enabled() {
            return Err(
                CreateMessageError::MessageTypeDisabled(message_type.code().clone()).into(),
            );
        }

        let schema = Schema::find(
            ctx.pool(),
            SchemasQueryBuilder::default()
                .message_type_id(Some(*message_type.id()))
                .version(Some(cmd.schema_version))
                .build()
                .unwrap(),
        )
        .await
        .tap_err(|e| tracing::error!("Failed to retrieve schema:{e}"))?
        .ok_or_else(|| CreateMessageError::SchemaNotFound(cmd.schema_version))?;

        if !schema.enabled() {
            return Err(CreateMessageError::SchemaDisabled(cmd.schema_version).into());
        }

        if !schema.published() {
            return Err(CreateMessageError::SchemaNotPublished(cmd.schema_version).into());
        }

        schema
            .value()
            .validate_payload(&cmd.payload)
            .tap_err(|e| tracing::error!("Invalid payload: {e}"))
            .map_err(CreateMessageError::from)?;

        Ok(MessageBuilder::default()
            .id(cmd.id)
            .status(Status::Pending)
            .payload(cmd.payload)
            .schema_id(*schema.id())
            .message_type_id(*schema.message_type_id())
            .business_unit_id(*schema.business_unit_id())
            .deliveries(Json(cmd.deliveries))
            .build()
            .unwrap()
            .save(ctx.tx())
            .await?)
    }
}
