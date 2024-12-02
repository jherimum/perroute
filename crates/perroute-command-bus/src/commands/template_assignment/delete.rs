use crate::bus::{
    Command, CommandBusContext, CommandHandler, CommandHandlerResult, CommandWrapper,
};
use bon::Builder;
use perroute_commons::{commands::CommandType, types::id::Id};
use perroute_storage::repository::TransactedRepository;

#[derive(Debug, thiserror::Error)]
pub enum DeleteTemplateAssignmentCommandError {}

#[derive(Debug, Clone, Builder)]
pub struct DeleteTemplateAssignmentCommand {
    id: Id,
}

impl Command for DeleteTemplateAssignmentCommand {
    type Output = ();

    fn command_type(&self) -> CommandType {
        CommandType::DeleteTemplateAssignment
    }

    fn to_event(
        &self,

        created_at: &perroute_commons::types::Timestamp,
        actor: &perroute_commons::types::actor::Actor,
        output: &Self::Output,
    ) -> perroute_commons::events::Event {
        todo!()
    }
}

pub struct DeleteTemplateAssignmentCommandHandler;

impl CommandHandler for DeleteTemplateAssignmentCommandHandler {
    type Command = DeleteTemplateAssignmentCommand;
    type Output = bool;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: CommandWrapper<'_, Self::Command>,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        todo!()
    }
}
