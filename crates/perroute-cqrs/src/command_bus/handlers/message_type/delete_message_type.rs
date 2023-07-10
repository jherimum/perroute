use crate::command_bus::{
    bus::CommandBusContext, commands::DeleteMessageTypeCommand, error::CommandBusError,
    handlers::CommandHandler,
};
use perroute_commons::types::id::Id;
use perroute_storage::models::message_type::{MessageType, MessageTypeQueryBuilder};

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
    type Output = bool;

    async fn handle<'tx, 'a>(
        &self,
        ctx: &mut CommandBusContext<'tx, 'a>,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        MessageType::find(
            ctx.tx(),
            MessageTypeQueryBuilder::default()
                .id(Some(*cmd.message_type_id()))
                .build()
                .unwrap(),
        )
        .await?
        .ok_or_else(|| DeleteMessageTypeError::MessageTypeNotFound(*cmd.message_type_id()))?
        .delete(ctx.tx())
        .await
        .map_err(CommandBusError::from)
    }
}
