use crate::{bus::Ctx, command::Command, error::CommandBusError};
use perroute_commons::types::{
    actor::Actor, code::Code, command_type::CommandType, id::Id, json_schema::InvalidPayloadError,
    payload::Payload,
};
use perroute_connectors::types::recipient::Recipient;
use perroute_storage::{
    models::{
        business_unit::{BusinessUnit, BusinessUnitQuery},
        message::{Message, MessageBuilder, Status},
        message_type::{MessageType, MessageTypeQuery},
    },
    query::FetchableModel,
};
use sqlx::types::Json;
use std::collections::HashSet;
use tap::TapFallible;
use thiserror::Error;

#[derive(Debug, derive_builder::Builder)]
pub struct CreateMessageCommand {
    id: Id,
    payload: Payload,
    business_unit_code: Code,
    message_type_code: Code,
    recipients: HashSet<Recipient>,
}

#[derive(Error, Debug)]
pub enum CreateMessageError {
    #[error("Business unit not found: {0}")]
    BusinessUnitNotFound(Code),

    #[error("Message type not found: {0}")]
    MessageTypeNotFound(Code),

    #[error(transparent)]
    InvalidPayload(#[from] InvalidPayloadError),
}

#[async_trait::async_trait]
impl Command for CreateMessageCommand {
    type Output = Message;

    #[tracing::instrument(name = "create_message_handler", skip(self, ctx))]
    async fn handle<'tx>(&self, ctx: &mut Ctx<'tx>) -> Result<Self::Output, CommandBusError> {
        let bu = BusinessUnit::find(
            ctx.pool(),
            BusinessUnitQuery::with_code(self.business_unit_code.clone()),
        )
        .await
        .tap_err(|e| tracing::error!("Failed to retrieve business unit:{e}"))?
        .ok_or_else(|| CreateMessageError::BusinessUnitNotFound(self.business_unit_code.clone()))?;

        let message_type = MessageType::find(
            ctx.pool(),
            MessageTypeQuery::with_code(self.message_type_code.clone()),
        )
        .await
        .tap_err(|e| tracing::error!("Failed to retrieve message type:{e}"))?
        .ok_or_else(|| CreateMessageError::MessageTypeNotFound(self.message_type_code.clone()))?;

        Ok(MessageBuilder::default()
            .id(self.id)
            .status(Status::Pending)
            .payload(self.payload.clone())
            .message_type_id(*message_type.id())
            .business_unit_id(*bu.id())
            .recipients(Json(self.recipients.clone()))
            .build()
            .unwrap()
            .save(ctx.pool())
            .await?)
    }

    fn supports(&self, _actor: &Actor) -> bool {
        true
    }

    fn command_type(&self) -> CommandType {
        CommandType::CreateMessage
    }
}
