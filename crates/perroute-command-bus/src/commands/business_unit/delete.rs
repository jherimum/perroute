use crate::bus::{
    Command, CommandBusContext, CommandHandler, CommandHandlerOutput, CommandHandlerResult,
};
use bon::Builder;
use perroute_commons::{commands::CommandType, events::Event, types::id::Id};
use perroute_storage::repository::{
    business_units::{BusinessUnitQuery, BusinessUnitRepository},
    TransactedRepository,
};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum DeleteBusinessUnitCommandError {
    #[error("Business unit not found")]
    BusinessUnitNotFound,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct DeleteBusinessUnitCommand {
    pub id: Id,
}

impl Command for DeleteBusinessUnitCommand {
    fn command_type(&self) -> CommandType {
        CommandType::DeleteBusinessUnit
    }

    fn to_event<R: TransactedRepository>(&self, ctx: &CommandBusContext<'_, R>) -> Event {
        todo!()
    }
}

pub struct DeleteBusinessUnitCommandHandler;

impl CommandHandler for DeleteBusinessUnitCommandHandler {
    type Command = DeleteBusinessUnitCommand;
    type Output = ();

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        let exists = BusinessUnitRepository::exists_business_unit(
            ctx.repository(),
            &BusinessUnitQuery::ById(cmd.id.clone()),
        )
        .await?;

        if !exists {
            return Err(DeleteBusinessUnitCommandError::BusinessUnitNotFound.into());
        }

        BusinessUnitRepository::delete_business_unit(ctx.repository(), &cmd.id).await?;

        CommandHandlerOutput::new(()).ok()
    }
}
