use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::Command,
    impl_command,
};
use perroute_commons::{events::TemplateAssignmentDeletedEvent, types::id::Id};

#[derive(Debug, thiserror::Error)]
pub enum DeleteTemplateAssignmentCommandError {}

impl_command!(DeleteTemplateAssignmentCommand, {
    template_assignment_id: Id,
});

pub struct DeleteTemplateAssignmentCommandHandler;

impl CommandHandler for DeleteTemplateAssignmentCommandHandler {
    type Command = DeleteTemplateAssignmentCommand;
    type Output = bool;
    type E = TemplateAssignmentDeletedEvent;

    async fn handle<
        C: AsRef<perroute_storage::active_record::datasource::Connection>,
    >(
        &self,
        ctx: &CommandBusContext<'_, C, Self::Command>,
    ) -> CommandHandlerResult<Self::Output> {
        todo!()
    }
}
