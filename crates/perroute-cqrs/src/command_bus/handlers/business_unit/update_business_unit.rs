use crate::{
    command,
    command_bus::{
        bus::CommandBusContext, commands::CommandType, error::CommandBusError,
        handlers::CommandHandler,
    },
    into_event,
};
use async_trait::async_trait;
use derive_new::new;
use perroute_commons::types::{actor::Actor, id::Id, vars::Vars};
use perroute_storage::{
    models::business_unit::{BusinessUnit, BusinessUnitQuery},
    query::FetchableModel,
};
use tap::TapFallible;

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("BusinessUnit with id {0} nor found")]
    BusinessUnitNotFound(Id),
}

command!(
    UpdateBusinessUnitCommand,
    CommandType::UpdateBusinessUnit,
    business_unit_id: Id,
    name: String,
    vars: Vars
);
into_event!(UpdateBusinessUnitCommand);

#[derive(Debug, new)]
pub struct UpdateBusinessUnitCommandHandler;

#[async_trait]
impl CommandHandler for UpdateBusinessUnitCommandHandler {
    type Command = UpdateBusinessUnitCommand;
    type Output = BusinessUnit;

    #[tracing::instrument(name = "update_business_units_handler", skip(self, ctx))]
    async fn handle<'ctx>(
        &self,
        ctx: &mut CommandBusContext<'ctx>,
        _: &Actor,
        command: Self::Command,
    ) -> Result<BusinessUnit, CommandBusError> {
        Ok(BusinessUnit::find(
            ctx.tx(),
            BusinessUnitQuery::with_id(command.business_unit_id),
        )
        .await
        .tap_err(|e| {
            tracing::error!(
                "Error while fetching business unit {}: {e}",
                command.business_unit_id
            );
        })?
        .ok_or(Error::BusinessUnitNotFound(command.business_unit_id))?
        .set_name(command.name.clone())
        .set_vars(command.vars.clone())
        .update(ctx.tx())
        .await
        .tap_err(|e| {
            tracing::error!(
                "Failed to update business unit {}: {e}",
                command.business_unit_id()
            );
        })?)
    }
}
