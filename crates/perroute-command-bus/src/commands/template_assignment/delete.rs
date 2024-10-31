use bon::Builder;
use perroute_commons::types::id::Id;
use perroute_storage::repository::TransactedRepository;
use crate::{bus::{Command, CommandBusContext, CommandHandler}, CommandBusResult};


#[derive(Debug, thiserror::Error)]
pub enum DeleteTemplateAssignmentCommandError{
}

#[derive(Debug, Clone, Builder)]
pub struct DeleteTemplateAssignmentCommand {
    id: Id,
    
}

impl Command for DeleteTemplateAssignmentCommand {
}


pub struct DeleteTemplateAssignmentCommandHandler;

impl CommandHandler for DeleteTemplateAssignmentCommandHandler {
    type Command = DeleteTemplateAssignmentCommand;
    type Output = bool;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<'_, R>,
    ) -> CommandBusResult<Self::Output> {
        todo!()
    }

}



