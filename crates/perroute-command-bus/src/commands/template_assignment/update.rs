use crate::{
    bus::{Command, CommandBusContext, CommandHandler},
    CommandBusResult,
};
use bon::Builder;
use perroute_commons::types::{vars::Vars, Priority, Timestamp};
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

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<'_, R>,
    ) -> CommandBusResult<Self::Output> {
        todo!()
    }
}