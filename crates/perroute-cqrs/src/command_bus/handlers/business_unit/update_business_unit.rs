use crate::{
    command,
    command_bus::{
        bus::CommandBusContext, commands::CommandType, handlers::CommandHandler, Result,
    },
    into_event,
};
use async_trait::async_trait;
use derive_new::new;
use perroute_commons::types::{id::Id, vars::Vars};
use perroute_storage::{
    models::business_unit::{BusinessUnit, BusinessUnitQuery},
    query::FetchableModel,
};
use sqlx::PgPool;
use tap::TapFallible;

#[derive(thiserror::Error, Debug, Clone)]
pub enum UpdateBusinessUnitError {
    #[error("BusinessUnit with id {0} nor found")]
    BusinessUnitNotFound(Id),
}

command!(
    UpdateBusinessUnitCommand,
    CommandType::UpdateBusinessUnit,
    business_unit_id: Id,
    name: Option<String>,
    vars: Option<Vars>
);
into_event!(UpdateBusinessUnitCommand);

#[derive(Debug)]
pub struct UpdateBusinessUnitCommandHandler {
    pool: PgPool,
}

impl UpdateBusinessUnitCommandHandler {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CommandHandler for UpdateBusinessUnitCommandHandler {
    type Command = UpdateBusinessUnitCommand;
    type Output = BusinessUnit;

    #[tracing::instrument(name = "update_business_units_handler", skip(self, ctx))]
    async fn handle<'ctx>(
        &self,
        ctx: &mut CommandBusContext,

        command: Self::Command,
    ) -> Result<BusinessUnit> {
        let mut bu = BusinessUnit::find(
            ctx.pool(),
            BusinessUnitQuery::with_id(command.business_unit_id),
        )
        .await
        .tap_err(|e| {
            tracing::error!(
                "Error while fetching business unit {}: {e}",
                command.business_unit_id
            );
        })?
        .ok_or(UpdateBusinessUnitError::BusinessUnitNotFound(
            command.business_unit_id,
        ))?;

        if command.name.is_none() && command.vars.is_none() {
            return Ok(bu);
        }

        if let Some(name) = command.name {
            bu = bu.set_name(name);
        }

        if let Some(vars) = command.vars {
            bu = bu.set_vars(vars);
        }

        Ok(bu.update(ctx.pool()).await.tap_err(|e| {
            tracing::error!(
                "Failed to update business unit {}: {e}",
                command.business_unit_id
            );
        })?)
    }
}
