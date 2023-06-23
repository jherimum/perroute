use crate::command_bus::{
    bus::CommandBusContext, commands::DeleteMessageTypeCommand, error::CommandBusError,
    handlers::CommandHandler,
};
use perroute_commons::types::id::Id;
use perroute_storage::models::message_type::MessageType;

#[derive(thiserror::Error, Debug, Clone)]
pub enum DeleteMessageTypeError {
    #[error("Message type with id {0} not found")]
    MessageTypeNotFound(Id),
}

#[derive(Debug)]
pub struct DeleteMessageTypeCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for DeleteMessageTypeCommandHandler {
    type Command = DeleteMessageTypeCommand;
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        cmd: Self::Command,
    ) -> Result<(), CommandBusError> {
        MessageType::find_by_id(ctx.tx(), cmd.message_type_id())
            .await?
            .ok_or(DeleteMessageTypeError::MessageTypeNotFound(
                *cmd.message_type_id(),
            ))?
            .delete(ctx.tx())
            .await
            .map_err(CommandBusError::from)
            .map(|_| ())
    }
}
