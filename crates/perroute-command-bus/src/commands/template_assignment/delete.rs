use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::Command,
};
use bon::Builder;
use perroute_commons::types::id::Id;
use perroute_storage::repository::TransactedRepository;

#[derive(Debug, thiserror::Error)]
pub enum DeleteTemplateAssignmentCommandError {}

#[derive(Debug, Clone, Builder)]
pub struct DeleteTemplateAssignmentCommand {
    id: Id,
}

impl Command for DeleteTemplateAssignmentCommand {
    fn event_type(&self) -> perroute_commons::events::EventType {
        todo!()
    }

    fn entity_id(&self) -> &Id {
        &self.id
    }
}

pub struct DeleteTemplateAssignmentCommandHandler;

impl CommandHandler for DeleteTemplateAssignmentCommandHandler {
    type Command = DeleteTemplateAssignmentCommand;
    type Output = bool;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &crate::commands::CommandWrapper<'_, Self::Command>,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        todo!()
    }
}
