use crate::command_bus::{
    bus::CommandBusContext, commands::UpdateMessageTypeCommand, error::CommandBusError,
    handlers::CommandHandler,
};
use perroute_commons::types::id::Id;
use perroute_storage::models::message_type::MessageType;

#[derive(Debug, thiserror::Error)]
pub enum UpdateMessageTypeError {
    #[error("Message type with id {0} not found")]
    MessageTypeNotFound(Id),
}

#[derive(Debug)]
pub struct UpdateMessageTypeCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for UpdateMessageTypeCommandHandler {
    type Command = UpdateMessageTypeCommand;
    async fn handle<'tx, 'a>(
        &self,
        ctx: &mut CommandBusContext<'tx, 'a>,
        cmd: Self::Command,
    ) -> Result<(), CommandBusError> {
        MessageType::find_by_id(ctx.tx(), cmd.message_type_id())
            .await?
            .ok_or(UpdateMessageTypeError::MessageTypeNotFound(
                *cmd.message_type_id(),
            ))?
            .set_description(cmd.description().clone())
            .set_enabled(*cmd.enabled())
            .update(ctx.tx())
            .await
            .map_err(CommandBusError::from)
            .map(|_| ())
    }
}
