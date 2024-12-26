use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::Command,
    impl_command,
};
use perroute_commons::{
    events::TemplateAssignmentCreatedEvent,
    types::{id::Id, priority::Priority, vars::Vars, Timestamp},
};
use perroute_storage::{
    models::template_assignment::TemplateAssignment,
    repository::TransactedRepository,
};

#[derive(Debug, thiserror::Error)]
pub enum CreateTemplateAssignmentCommandError {}

impl_command!(CreateTemplateAssignmentCommand,{
    template_assignment_id: Id,
    business_unit_id: Id,
     message_type_id: Id,
     vars: Vars,
     priority: Priority,
     start_at: Timestamp,
     end_at: Option<Timestamp>,
     enabled: bool,
});

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
