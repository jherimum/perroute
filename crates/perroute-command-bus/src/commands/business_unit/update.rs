use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerOutput, CommandHandlerResult},
    commands::Command,
    impl_command, CommandBusError,
};
use perroute_commons::{
    events::BusinessUnitUpdatedEvent,
    types::{id::Id, name::Name, vars::Vars},
};
use perroute_storage::{
    models::business_unit::BusinessUnit,
    repository::{
        business_units::{BusinessUnitQuery, BusinessUnitRepository},
        TransactedRepository,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum UpdateBusinessUnitCommandError {
    #[error("Business unit not found")]
    NotFound,
}

impl_command!(UpdateBusinessUnitCommand, {
    business_unit_id: Id,
    name: Name,
    vars: Vars,
});

pub struct UpdateBusinessUnitCommandHandler;

impl CommandHandler for UpdateBusinessUnitCommandHandler {
    type Command = UpdateBusinessUnitCommand;
    type Output = BusinessUnit;
    type ApplicationEvent = BusinessUnitUpdatedEvent;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &crate::commands::CommandWrapper<'_, Self::Command>,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output, Self::ApplicationEvent> {
        let bu = match BusinessUnitRepository::find_business_unit(
            ctx.repository(),
            &BusinessUnitQuery::ById(cmd.inner().business_unit_id.clone()),
        )
        .await?
        {
            Some(bu) => {
                let bu = BusinessUnitRepository::update_business_unit(
                    ctx.repository(),
                    bu.set_name(cmd.inner().name.clone())
                        .set_vars(cmd.inner().vars.clone())
                        .set_updated_at(cmd.created_at().clone()),
                )
                .await?;
                Ok(bu)
            }
            None => Err(CommandBusError::from(
                UpdateBusinessUnitCommandError::NotFound,
            )),
        }?;

        Ok(CommandHandlerOutput::new(
            bu,
            BusinessUnitUpdatedEvent::builder()
                .business_unit_id(cmd.command.business_unit_id.clone())
                .name(cmd.command.name.clone())
                .vars(cmd.command.vars.clone())
                .build(),
        ))
    }
}
