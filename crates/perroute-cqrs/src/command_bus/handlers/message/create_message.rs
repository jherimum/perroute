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
    code::Code, id::Id, json_schema::InvalidPayloadError, payload::Payload, version::Version,
};
use perroute_connectors::types::delivery::Delivery;
use perroute_messaging::events::EventType;
use perroute_storage::{
    models::{
        business_unit::{BusinessUnit, BusinessUnitQueryBuilder},
        message::{Message, MessageBuilder, Status},
        message_type::{MessageType, MessageTypeQueryBuilder},
    },
    query::FetchableModel,
};
use serde::Serialize;
use sqlx::{types::Json, PgPool};
use tap::TapFallible;
use thiserror::Error;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct CreateMessageCommand {
    id: Id,
    payload: Payload,
    business_unit_code: Code,
    message_type_code: Code,
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

    #[error("Message type {0} is disabled")]
    MessageTypeDisabled(Code),

    #[error(transparent)]
    InvalidPayload(#[from] InvalidPayloadError),
}

#[derive(Debug)]
pub struct CreateMessageCommandHandler {
    pool: PgPool,
}

impl CreateMessageCommandHandler {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl CommandHandler for CreateMessageCommandHandler {
    type Command = CreateMessageCommand;
    type Output = Message;

    #[tracing::instrument(name = "create_message_handler", skip(self, ctx))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext,
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

        // Ok(MessageBuilder::default()
        //     .id(cmd.id)
        //     .status(Status::Pending)
        //     .payload(cmd.payload)
        //     .message_type_id(*schema.message_type_id())
        //     .business_unit_id(*schema.business_unit_id())
        //     .deliveries(Json(cmd.deliveries))
        //     .build()
        //     .unwrap()
        //     .save(ctx.pool())
        //     .await?)

        todo!()
    }
}
