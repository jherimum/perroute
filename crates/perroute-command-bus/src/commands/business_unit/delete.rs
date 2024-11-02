use crate::bus::{
    Command, CommandBusContext, CommandHandler, CommandHandlerOutput, CommandHandlerResult,
};
use bon::Builder;
use perroute_commons::{commands::CommandType, events::Event, types::id::Id};
use perroute_storage::repository::{business_units::BusinessUnitRepository, TransactedRepository};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum DeleteBusinessUnitCommandError {}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct DeleteBusinessUnitCommand {
    pub id: Id,
}

impl Command for DeleteBusinessUnitCommand {
    fn command_type(&self) -> CommandType {
        CommandType::DeleteBusinessUnit
    }
}

pub struct DeleteBusinessUnitCommandHandler;

impl CommandHandler for DeleteBusinessUnitCommandHandler {
    type Command = DeleteBusinessUnitCommand;
    type Output = bool;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        let deleted =
            BusinessUnitRepository::delete_business_unit(ctx.repository(), &cmd.id).await?;

        CommandHandlerOutput::new(deleted)
            .with_event(Event::BusinessUnitDeleted(cmd.id.clone()))
            .ok()
    }
}
