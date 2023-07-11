use crate::command_bus::{
    bus::CommandBusContext, commands::DeleteMessageTypeCommand, error::CommandBusError,
    handlers::CommandHandler,
};
use perroute_commons::types::{actor::Actor, id::Id};
use perroute_storage::{
    models::message_type::{MessageType, MessageTypeQueryBuilder},
    query::FetchableModel,
};

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

    #[tracing::instrument(name = "delete_message_type_handler", skip(self, ctx))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        _: &Actor,
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
