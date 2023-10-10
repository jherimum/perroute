use crate::{bus::Ctx, command::Command, error::CommandBusError};
use perroute_commons::types::{actor::Actor, command_type::CommandType, id::Id};
use perroute_storage::{
    models::message_type::{MessageType, MessageTypeQuery},
    query::FetchableModel,
};
use tap::TapFallible;

#[derive(Debug)]
pub struct DeleteMessageTypeCommand {
    id: Id,
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum DeleteMessageTypeError {
    #[error("Message type with id {0} not found")]
    MessageTypeNotFound(Id),

    #[error("Message type {0} could not be deleted:{1}")]
    MessageTypeDelete(Id, &'static str),
}

#[async_trait::async_trait]
impl Command for DeleteMessageTypeCommand {
    type Output = bool;

    #[tracing::instrument(name = "delete_message_type_handler", skip(self, ctx))]
    async fn handle<'tx>(&self, ctx: &mut Ctx<'tx>) -> Result<Self::Output, CommandBusError> {
        let message_type = MessageType::find(ctx.pool(), MessageTypeQuery::with_id(self.id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrive message type {}: {e}", self.id))?
            .ok_or(DeleteMessageTypeError::MessageTypeNotFound(self.id))?;

        Ok(message_type
            .delete(ctx.pool())
            .await
            .tap_err(|e| tracing::error!("Failed to delete message type {}:{e}", self.id))?)
    }

    fn command_type(&self) -> CommandType {
        CommandType::DeleteMessageType
    }

    fn supports(&self, _actor: &Actor) -> bool {
        true
    }
}
