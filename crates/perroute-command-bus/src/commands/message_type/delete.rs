use crate::bus::{
    Command, CommandBusContext, CommandHandler, CommandHandlerOutput, CommandHandlerResult,
};
use bon::Builder;
use perroute_commons::{commands::CommandType, types::id::Id};
use perroute_storage::repository::{message_types::MessageTypeRepository, TransactedRepository};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum DeleteMessageTypeCommandError {}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct DeleteMessageTypeCommand {
    id: Id,
}

impl Command for DeleteMessageTypeCommand {
    fn command_type(&self) -> CommandType {
        CommandType::DeleteMessageType
    }

    fn to_event<R: TransactedRepository>(
        &self,
        ctx: &CommandBusContext<'_, R>,
    ) -> perroute_commons::events::Event {
        todo!()
    }
}

pub struct DeleteMessageTypeCommandHandler;

impl CommandHandler for DeleteMessageTypeCommandHandler {
    type Command = DeleteMessageTypeCommand;
    type Output = bool;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        let deleted = MessageTypeRepository::delete_message_type(ctx.repository(), &cmd.id).await?;

        CommandHandlerOutput::new(deleted).ok()
    }
}
