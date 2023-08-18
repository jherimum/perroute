use crate::{
    command,
    command_bus::{
        bus::CommandBusContext, commands::CommandType, error::CommandBusError,
        handlers::CommandHandler,
    },
    into_event,
};
use async_trait::async_trait;
use perroute_commons::types::{actor::Actor, id::Id};
use perroute_storage::{
    models::{
        business_unit::{BusinessUnit, BusinessUnitQuery},
        channel::{Channel, ChannelQuery},
        message_type::{MessageType, MessageTypeQuery},
    },
    query::FetchableModel,
};
use tap::TapFallible;

#[derive(thiserror::Error, Debug, Clone)]
pub enum DeleteBusinessUnitCommandHandlerError {
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
pub struct DeleteBusinessUnitCommandHandler;

#[async_trait]
impl CommandHandler for DeleteBusinessUnitCommandHandler {
    type Command = DeleteBusinessUnitCommand;
    type Output = bool;

    #[tracing::instrument(name = "delete_business_unit_handler", skip(self, ctx))]
    async fn handle<'ctx>(
        &self,
        ctx: &mut CommandBusContext<'ctx>,
        _: &Actor,
        command: Self::Command,
    ) -> Result<bool, CommandBusError> {
        let bu = BusinessUnit::find(
            ctx.tx(),
            BusinessUnitQuery::with_id(command.business_unit_id),
        )
        .await
        .tap_err(|e| tracing::error!("Failed to retrieve business unit: {e}"))?
        .ok_or(DeleteBusinessUnitCommandHandlerError::BusinessUnitNotFound(
            command.business_unit_id,
        ))?;

        if Channel::exists(ctx.pool(), ChannelQuery::from_business_unit(*bu.id()))
            .await
            .tap_err(|e| tracing::error!("Failed to check exist channels: {e}"))?
        {
            return Err(DeleteBusinessUnitCommandHandlerError::BusinessUnitDelete(
                *bu.id(),
                "There are channles associated with this Business unit",
            )
            .into());
        }

        if MessageType::exists(ctx.pool(), MessageTypeQuery::from_business_unit(*bu.id()))
            .await
            .tap_err(|e| tracing::error!("Failed to check exist message types: {e}"))?
        {
            return Err(DeleteBusinessUnitCommandHandlerError::BusinessUnitDelete(
                *bu.id(),
                "There are message types associated with this Business unit",
            )
            .into());
        }

        Ok(bu
            .delete(ctx.tx())
            .await
            .tap_err(|e| tracing::error!("Failed to delete business unit: {e}"))?)
    }
}
