use crate::{bus::Ctx, command::Command, error::CommandBusError};
use async_trait::async_trait;
use perroute_commons::types::{actor::Actor, command_type::CommandType, id::Id, vars::Vars};
use perroute_storage::{
    models::business_unit::{BusinessUnit, BusinessUnitQuery},
    query::FetchableModel,
};
use tap::TapFallible;

#[derive(thiserror::Error, Debug, Clone)]
pub enum UpdateBusinessUnitError {
    #[error("BusinessUnit with id {0} nor found")]
    BusinessUnitNotFound(Id),
}

#[derive(Debug, derive_builder::Builder)]
pub struct UpdateBusinessUnitCommand {
    business_unit_id: Id,
    name: Option<String>,
    vars: Option<Vars>,
}

#[async_trait]
impl Command for UpdateBusinessUnitCommand {
    type Output = BusinessUnit;

    #[tracing::instrument(name = "update_business_units_handler", skip(self, ctx))]
    async fn handle<'ctx>(&self, ctx: &mut Ctx<'ctx>) -> Result<BusinessUnit, CommandBusError> {
        let mut bu = BusinessUnit::find(
            ctx.pool(),
            BusinessUnitQuery::with_id(self.business_unit_id),
        )
        .await
        .tap_err(|e| {
            tracing::error!(
                "Error while fetching business unit {}: {e}",
                self.business_unit_id
            );
        })?
        .ok_or(UpdateBusinessUnitError::BusinessUnitNotFound(
            self.business_unit_id,
        ))?;

        if self.name.is_none() && self.vars.is_none() {
            return Ok(bu);
        }

        if let Some(name) = &self.name {
            bu = bu.set_name(name);
        }

        if let Some(vars) = &self.vars {
            bu = bu.set_vars(vars.clone());
        }

        Ok(bu.update(ctx.pool()).await.tap_err(|e| {
            tracing::error!(
                "Failed to update business unit {}: {e}",
                self.business_unit_id
            );
        })?)
    }

    fn command_type(&self) -> CommandType {
        CommandType::UpdateBusinessUnit
    }

    fn supports(&self, _actor: &Actor) -> bool {
        true
    }
}
