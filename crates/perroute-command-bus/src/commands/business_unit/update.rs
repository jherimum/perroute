use crate::{
    bus::{Command, CommandBusContext, CommandHandler},
    CommandBusError, CommandBusResult,
};
use bon::Builder;
use perroute_commons::types::{id::Id, vars::Vars, Name};
use perroute_storage::{
    models::business_unit::BusinessUnit,
    repository::{business_units::BusinessUnitRepository, TransactedRepository},
};

#[derive(Debug, thiserror::Error)]
pub enum UpdateBusinessUnitCommandError {
    #[error("Business unit not found")]
    NotFound,
}

#[derive(Debug, Clone, Builder)]
pub struct UpdateBusinessUnitCommand {
    pub id: Id,
    pub name: Name,
    pub vars: Option<Vars>,
}

impl Command for UpdateBusinessUnitCommand {}

pub struct UpdateBusinessUnitCommandHandler;

impl CommandHandler for UpdateBusinessUnitCommandHandler {
    type Command = UpdateBusinessUnitCommand;
    type Output = BusinessUnit;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<'_, R>,
    ) -> CommandBusResult<Self::Output> {
        match BusinessUnitRepository::find_business_unit(ctx.repository(), &cmd.id).await? {
            Some(bu) => {
                let bu = BusinessUnitRepository::update_business_unit(
                    ctx.repository(),
                    bu.set_name(cmd.name.clone()).set_vars(cmd.vars.clone()),
                )
                .await?;
                Ok(bu)
            }
            None => Err(CommandBusError::from(
                UpdateBusinessUnitCommandError::NotFound,
            )),
        }
    }
}