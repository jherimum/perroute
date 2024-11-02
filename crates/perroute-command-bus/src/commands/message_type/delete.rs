use crate::bus::{
    Command, CommandBusContext, CommandHandler, CommandHandlerOutput, CommandHandlerResult,
};
use bon::Builder;
use perroute_commons::types::id::Id;
use perroute_storage::repository::{message_types::MessageTypeRepository, TransactedRepository};

#[derive(Debug, thiserror::Error)]
pub enum DeleteMessageTypeCommandError {}

#[derive(Debug, Clone, Builder)]
pub struct DeleteMessageTypeCommand {
    id: Id,
}

impl Command for DeleteMessageTypeCommand {}

pub struct DeleteMessageTypeCommandHandler;

impl CommandHandler for DeleteMessageTypeCommandHandler {
    type Command = DeleteMessageTypeCommand;
    type Output = bool;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        Ok(CommandHandlerOutput::new(
            MessageTypeRepository::delete_message_type(ctx.repository(), &cmd.id).await?,
            None,
        ))
    }
}
