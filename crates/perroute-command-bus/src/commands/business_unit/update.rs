use crate::{
    bus::{Command, CommandBusContext, CommandHandler, CommandHandlerOutput, CommandHandlerResult},
    commands::CommandType,
    CommandBusError,
};
use bon::Builder;
use perroute_commons::types::{id::Id, name::Name, vars::Vars};
use perroute_events::event::Event;
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

impl Command for UpdateBusinessUnitCommand {
    fn command_type(&self) -> CommandType {
        CommandType::UpdateBusinessUnit
    }
}

pub struct UpdateBusinessUnitCommandHandler;

impl CommandHandler for UpdateBusinessUnitCommandHandler {
    type Command = UpdateBusinessUnitCommand;
    type Output = BusinessUnit;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        let bu = match BusinessUnitRepository::find_business_unit(ctx.repository(), &cmd.id).await?
        {
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
        }?;

        CommandHandlerOutput::new(bu.clone())
            .with_event(Event::BusinessUnitUpdated(bu.id().clone()))
            .ok()
    }
}
