use crate::{
    bus::{Command, CommandBusContext, CommandHandler, CommandHandlerResult},
    CommandBusResult,
};
use bon::Builder;
use perroute_commons::types::{priority::Priority, vars::Vars, Timestamp};
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

impl Command for UpdateTemplateAssignmentCommand {}

pub struct UpdateTemplateAssignmentCommandHandler;

impl CommandHandler for UpdateTemplateAssignmentCommandHandler {
    type Command = UpdateTemplateAssignmentCommand;
    type Output = TemplateAssignment;
    type Event = ();

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output, Self::Event> {
        todo!()
    }
}
