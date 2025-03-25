use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::Command,
    impl_command, CommandBusError,
};
use perroute_commons::{
    events::BusinessUnitUpdatedEvent,
    types::{id::Id, name::Name, vars::Vars},
};
use perroute_storage::{
    active_record::{business_unit::BusinessUnitQuery, ActiveRecord},
    models::business_unit::BusinessUnit,
};
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum UpdateBusinessUnitCommandError {
    #[error("Business unit not found")]
    NotFound,
}

impl_command!(UpdateBusinessUnitCommand, {
    business_unit_id: Id,
    name: Option<Name>,
    vars: Option<Vars>,
});

pub struct UpdateBusinessUnitCommandHandler;

impl CommandHandler for UpdateBusinessUnitCommandHandler {
    type Command = UpdateBusinessUnitCommand;
    type Output = BusinessUnit;
    type E = BusinessUnitUpdatedEvent;

    async fn handle<
        C: AsRef<perroute_storage::active_record::datasource::Connection>,
    >(
        &self,
        ctx: &CommandBusContext<'_, C, Self::Command>,
    ) -> CommandHandlerResult<Self::Output> {
        let mut bu = BusinessUnit::fetch_optional(
            ctx.datasource(),
            BusinessUnitQuery::ById(&ctx.command().business_unit_id),
        )
        .await
        .tap_err(|e| {
            log::error!("Error checking if business unit exists: {:?}", e)
        })?
        .ok_or(UpdateBusinessUnitCommandError::NotFound)?;

        if let Some(name) = ctx.command().name.as_ref() {
            bu = bu.set_name(name);
        }

        if let Some(vars) = ctx.command().vars.as_ref() {
            bu = bu.set_vars(vars);
        }

        bu = bu.set_updated_at(ctx.timestamp());

        bu.update(ctx.datasource())
            .await
            .tap_err(|e| log::error!("Failed to update business unit: {e}"))
            .map_err(CommandBusError::from)
    }
}
