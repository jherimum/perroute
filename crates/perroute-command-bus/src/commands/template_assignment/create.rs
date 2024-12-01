use crate::bus::{Command, CommandBusContext, CommandHandler, CommandHandlerResult};
use bon::Builder;
use perroute_commons::{
    commands::CommandType,
    types::{id::Id, priority::Priority, vars::Vars, Timestamp},
};
use perroute_storage::{
    models::template_assignment::TemplateAssignment, repository::TransactedRepository,
};

#[derive(Debug, thiserror::Error)]
pub enum CreateTemplateAssignmentCommandError {}

#[derive(Debug, Clone, Builder)]
pub struct CreateTemplateAssignmentCommand {
    id: Id,
    pub business_unit_id: Id,
    pub message_type_id: Id,
    pub vars: Vars,
    pub priority: Priority,
    pub start_at: Timestamp,
    pub end_at: Option<Timestamp>,
    pub enabled: bool,
}

impl Command for CreateTemplateAssignmentCommand {
    fn command_type(&self) -> CommandType {
        CommandType::CreateTemplateAssignment
    }

    fn to_event(
        &self,
        actor: &perroute_commons::types::actor::Actor,
    ) -> perroute_commons::events::Event {
        todo!()
    }
}

pub struct CreateTemplateAssignmentCommandHandler;

impl CommandHandler for CreateTemplateAssignmentCommandHandler {
    type Command = CreateTemplateAssignmentCommand;
    type Output = TemplateAssignment;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        todo!()
    }
}
