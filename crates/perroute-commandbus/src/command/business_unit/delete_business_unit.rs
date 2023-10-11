use crate::{bus::Ctx, command::Command, error::CommandBusError};
use async_trait::async_trait;
use perroute_commons::types::{actor::Actor, command_type::CommandType, id::Id};
use perroute_storage::{
    models::{
        business_unit::{BusinessUnit, BusinessUnitQuery},
        channel::{Channel, ChannelQuery},
    },
    query::FetchableModel,
};
use tap::TapFallible;

#[derive(thiserror::Error, Debug, Clone)]
pub enum DeleteBusinessUnitError {
    #[error("BusinessUnit with id {0} nor found")]
    BusinessUnitNotFound(Id),

    #[error("Business Unit {0} could not be deleted: {1}")]
    BusinessUnitDelete(Id, &'static str),
}

#[derive(Debug, derive_builder::Builder)]
pub struct DeleteBusinessUnitCommand {
    business_unit_id: Id,
}

#[async_trait]
impl Command for DeleteBusinessUnitCommand {
    type Output = bool;

    #[tracing::instrument(name = "delete_business_unit_handler", skip(self, ctx))]
    async fn handle<'ctx>(&self, ctx: &mut Ctx<'ctx>) -> Result<bool, CommandBusError> {
        let bu = BusinessUnit::find(
            ctx.pool(),
            BusinessUnitQuery::with_id(self.business_unit_id),
        )
        .await
        .tap_err(|e| {
            tracing::error!(
                "Failed to retrieve business unit {}: {e}",
                self.business_unit_id
            )
        })?
        .ok_or(DeleteBusinessUnitError::BusinessUnitNotFound(
            self.business_unit_id,
        ))?;

        if Channel::exists(ctx.pool(), ChannelQuery::with_business_unit(*bu.id()))
            .await
            .tap_err(|e| tracing::error!("Failed to check exist channels: {e}"))?
        {
            return Err(DeleteBusinessUnitError::BusinessUnitDelete(
                *bu.id(),
                "There are channels associated with this Business unit",
            )
            .into());
        }

        //todo: check if there are messages associated with this business unit
        //todo: check if there are template assignments with this business unit

        Ok(bu
            .delete(ctx.pool())
            .await
            .tap_err(|e| tracing::error!("Failed to delete business unit: {e}"))?)
    }

    fn command_type(&self) -> CommandType {
        CommandType::DeleteBusinessUnit
    }

    fn supports(&self, actor: &Actor) -> bool {
        true
    }
}
