use bon::Builder;
use perroute_commons::types::id::Id;
use perroute_storage::repository::TransactedRepository;
use crate::{bus::{Command, CommandBusContext, CommandHandler}, CommandBusResult};

#[derive(Debug, thiserror::Error)]
pub enum DeleteMessageTypeCommandError{
}

#[derive(Debug, Clone, Builder)]
pub struct DeleteMessageTypeCommand {
    id: Id
}

impl Command for DeleteMessageTypeCommand {
}


pub struct DeleteMessageTypeCommandHandler;

impl CommandHandler for DeleteMessageTypeCommandHandler {
    type Command = DeleteMessageTypeCommand;
    type Output = bool;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<'_, R>,
    ) -> CommandBusResult<Self::Output> {
        todo!()
    }

}



