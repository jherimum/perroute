use crate::{
    command,
    command_bus::{
        bus::CommandBusContext, commands::CommandType, handlers::CommandHandler, Result,
    },
    into_event,
};
use async_trait::async_trait;
use perroute_commons::types::id::Id;
use perroute_storage::{
    models::{
        business_unit::{BusinessUnit, BusinessUnitQuery},
        channel::{Channel, ChannelQuery},
    },
    query::FetchableModel,
};
use sqlx::PgPool;
use tap::TapFallible;

#[derive(thiserror::Error, Debug, Clone)]
pub enum DeleteBusinessUnitError {
    #[error("BusinessUnit with id {0} nor found")]
    BusinessUnitNotFound(Id),

    #[error("Business Unit {0} could not be deleted: {1}")]
    BusinessUnitDelete(Id, &'static str),
}

command!(
    DeleteBusinessUnitCommand,
    CommandType::DeleteBusinessUnit,
    business_unit_id: Id
);
into_event!(DeleteBusinessUnitCommand);

#[derive(Debug)]
pub struct DeleteBusinessUnitCommandHandler {
    pool: PgPool,
}

impl DeleteBusinessUnitCommandHandler {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CommandHandler for DeleteBusinessUnitCommandHandler {
    type Command = DeleteBusinessUnitCommand;
    type Output = bool;

    #[tracing::instrument(name = "delete_business_unit_handler", skip(self, ctx))]
    async fn handle<'ctx>(&self, ctx: &mut CommandBusContext, cmd: Self::Command) -> Result<bool> {
        let bu = BusinessUnit::find(ctx.pool(), BusinessUnitQuery::with_id(cmd.business_unit_id))
            .await
            .tap_err(|e| {
                tracing::error!(
                    "Failed to retrieve business unit {}: {e}",
                    cmd.business_unit_id
                )
            })?
            .ok_or(DeleteBusinessUnitError::BusinessUnitNotFound(
                cmd.business_unit_id,
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
}
