use crate::{
    command,
    command_bus::{
        bus::CommandBusContext, commands::CommandType, handlers::CommandHandler, Result,
    },
    into_event,
};
use perroute_commons::types::id::Id;
use perroute_storage::{
    models::message_type::{MessageType, MessageTypeQuery},
    query::FetchableModel,
};
use sqlx::PgPool;
use tap::TapFallible;

command!(
    DeleteMessageTypeCommand,
    CommandType::DeleteMessageType,
    id: Id
);
into_event!(DeleteMessageTypeCommand);

#[derive(thiserror::Error, Debug, Clone)]
pub enum DeleteMessageTypeError {
    #[error("Message type with id {0} not found")]
    MessageTypeNotFound(Id),

    #[error("Message type {0} could not be deleted:{1}")]
    MessageTypeDelete(Id, &'static str),
}

#[derive(Debug)]
pub struct DeleteMessageTypeCommandHandler {
    pool: PgPool,
}

impl DeleteMessageTypeCommandHandler {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl CommandHandler for DeleteMessageTypeCommandHandler {
    type Command = DeleteMessageTypeCommand;
    type Output = bool;

    #[tracing::instrument(name = "delete_message_type_handler", skip(self, ctx))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext,

        cmd: Self::Command,
    ) -> Result<Self::Output> {
        let message_type = MessageType::find(ctx.pool(), MessageTypeQuery::with_id(cmd.id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrive message type {}: {e}", cmd.id))?
            .ok_or(DeleteMessageTypeError::MessageTypeNotFound(cmd.id))?;

        Ok(message_type
            .delete(ctx.pool())
            .await
            .tap_err(|e| tracing::error!("Failed to delete message type {}:{e}", cmd.id))?)
    }
}
