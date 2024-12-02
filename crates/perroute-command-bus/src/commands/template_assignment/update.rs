use crate::bus::{
    Command, CommandBusContext, CommandHandler, CommandHandlerResult, CommandWrapper,
};
use bon::Builder;
use perroute_commons::{
    commands::CommandType,
    types::{priority::Priority, vars::Vars, Timestamp},
};
use perroute_storage::{
    models::template_assignment::TemplateAssignment, repository::TransactedRepository,
};

#[derive(Debug, thiserror::Error)]
pub enum UpdateTemplateAssignmentCommandError {}

#[derive(Debug, Clone, Builder)]
pub struct UpdateTemplateAssignmentCommand {
    pub vars: Vars,
    pub priority: Priority,
    pub start_at: Timestamp,
    pub end_at: Option<Timestamp>,
    pub enabled: bool,
}

impl Command for UpdateTemplateAssignmentCommand {
    type Output = TemplateAssignment;

    fn command_type(&self) -> CommandType {
        CommandType::UpdateTemplateAssignment
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

pub struct UpdateTemplateAssignmentCommandHandler;

impl CommandHandler for UpdateTemplateAssignmentCommandHandler {
    type Command = UpdateTemplateAssignmentCommand;
    type Output = TemplateAssignment;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: CommandWrapper<'_, Self::Command>,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        todo!()
    }
}
