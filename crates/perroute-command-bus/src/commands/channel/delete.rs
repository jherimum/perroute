use bon::Builder;
use perroute_commons::types::id::Id;
use perroute_storage::repository::TransactedRepository;
use crate::{bus::{Command, CommandBusContext, CommandHandler}, CommandBusResult};


#[derive(Debug, thiserror::Error)]
pub enum DeleteChannelCommandError{
}

#[derive(Debug, Clone, Builder)]
pub struct DeleteChannelCommand {
    id: Id,
    
}

impl Command for DeleteChannelCommand {
}


pub struct DeleteChannelCommandHandler;

impl CommandHandler for DeleteChannelCommandHandler {
    type Command = DeleteChannelCommand;
    type Output = bool;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<'_, R>,
    ) -> CommandBusResult<Self::Output> {
        todo!()
    }

}



