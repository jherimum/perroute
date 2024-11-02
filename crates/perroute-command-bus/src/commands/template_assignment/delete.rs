use crate::{
    bus::{Command, CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::CommandType,
};
use bon::Builder;
use perroute_commons::types::id::Id;
use perroute_storage::repository::TransactedRepository;

#[derive(Debug, thiserror::Error)]
pub enum DeleteTemplateAssignmentCommandError {}

#[derive(Debug, Clone, Builder)]
pub struct DeleteTemplateAssignmentCommand {
    id: Id,
}

impl Command for DeleteTemplateAssignmentCommand {
    fn command_type(&self) -> CommandType {
        CommandType::DeleteTemplateAssignment
    }
}

pub struct DeleteTemplateAssignmentCommandHandler;

impl CommandHandler for DeleteTemplateAssignmentCommandHandler {
    type Command = DeleteTemplateAssignmentCommand;
    type Output = bool;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        todo!()
    }
}
