use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::Command,
};
use bon::Builder;
use perroute_commons::{
    events::TemplateAssignmentCreatedEvent,
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
    fn event_type(&self) -> perroute_commons::events::EventType {
        todo!()
    }

    fn entity_id(&self) -> &Id {
        &self.id
    }
}

pub struct CreateTemplateAssignmentCommandHandler;

impl CommandHandler for CreateTemplateAssignmentCommandHandler {
    type Command = CreateTemplateAssignmentCommand;
    type Output = TemplateAssignment;
    type ApplicationEvent = TemplateAssignmentCreatedEvent;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &crate::commands::CommandWrapper<'_, Self::Command>,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output, Self::ApplicationEvent> {
        todo!()
    }
}
