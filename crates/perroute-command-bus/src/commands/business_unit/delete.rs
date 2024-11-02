use crate::bus::{
    Command, CommandBusContext, CommandHandler, CommandHandlerOutput, CommandHandlerResult,
};
use bon::Builder;
use perroute_commons::types::id::Id;
use perroute_storage::repository::{business_units::BusinessUnitRepository, TransactedRepository};

#[derive(Debug, thiserror::Error)]
pub enum DeleteBusinessUnitCommandError {}

#[derive(Debug, Clone, Builder)]
pub struct DeleteBusinessUnitCommand {
    pub id: Id,
}

impl Command for DeleteBusinessUnitCommand {}

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

        Ok(CommandHandlerOutput::new(deleted, None))
    }
}
