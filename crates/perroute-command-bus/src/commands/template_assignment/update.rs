use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::Command,
    impl_command,
};
use perroute_commons::{
    events::TemplateAssignmentUpdatedEvent,
    types::{id::Id, priority::Priority, vars::Vars, Timestamp},
};
use perroute_storage::{models::template_assignment::TemplateAssignment};

#[derive(Debug, thiserror::Error)]
pub enum UpdateTemplateAssignmentCommandError {}

impl_command!(UpdateTemplateAssignmentCommand, {
    template_assignment_id: Id,
    vars: Vars,
    priority: Priority,
    start_at: Timestamp,
    end_at: Option<Timestamp>,
    enabled: bool,
});

pub struct UpdateTemplateAssignmentCommandHandler;

impl CommandHandler for UpdateTemplateAssignmentCommandHandler {
    type Command = UpdateTemplateAssignmentCommand;
    type Output = TemplateAssignment;
    type E = TemplateAssignmentUpdatedEvent;

    async fn handle<
        C: AsRef<perroute_storage::active_record::datasource::Connection>,
    >(
        &self,
        ctx: &CommandBusContext<'_, C, Self::Command>,
    ) -> CommandHandlerResult<Self::Output> {
        todo!()
    }
}
