use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::Command,
};
use bon::Builder;
use perroute_commons::types::{id::Id, priority::Priority, vars::Vars, Timestamp};
use perroute_storage::{
    models::template_assignment::TemplateAssignment, repository::TransactedRepository,
};

#[derive(Debug, thiserror::Error)]
pub enum UpdateTemplateAssignmentCommandError {}

#[derive(Debug, Clone, Builder)]
pub struct UpdateTemplateAssignmentCommand {
    pub id: Id,
    pub vars: Vars,
    pub priority: Priority,
    pub start_at: Timestamp,
    pub end_at: Option<Timestamp>,
    pub enabled: bool,
}

impl Command for UpdateTemplateAssignmentCommand {
    fn event_type(&self) -> perroute_commons::events::EventType {
        todo!()
    }

    fn entity_id(&self) -> &Id {
        &self.id
    }
}

pub struct UpdateTemplateAssignmentCommandHandler;

impl CommandHandler for UpdateTemplateAssignmentCommandHandler {
    type Command = UpdateTemplateAssignmentCommand;
    type Output = TemplateAssignment;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &crate::commands::CommandWrapper<'_, Self::Command>,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        todo!()
    }
}
