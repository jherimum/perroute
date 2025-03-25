use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::Command,
    impl_command, CommandBusError,
};
use perroute_commons::{events::BusinessUnitDeletedEvent, types::id::Id};
use perroute_storage::{
    active_record::{
        business_unit::BusinessUnitQuery, channel::ChannelQuery, ActiveRecord,
    },
    models::{business_unit::BusinessUnit, channel::Channel},
};
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum DeleteBusinessUnitCommandError {
    #[error("Business unit not found")]
    BusinessUnitNotFound,

    #[error("Business unit has channels")]
    BusinessUnitHasChannels,
}

impl_command!(DeleteBusinessUnitCommand, {
    business_unit_id: Id
});

pub struct DeleteBusinessUnitCommandHandler;

impl CommandHandler for DeleteBusinessUnitCommandHandler {
    type Command = DeleteBusinessUnitCommand;
    type Output = bool;
    type E = BusinessUnitDeletedEvent;

    fn into_event(
        command: &Self::Command,
        output: &Self::Output,
    ) -> Option<BusinessUnitDeletedEvent> {
        match output {
            true => Some(
                BusinessUnitDeletedEvent::builder()
                    .business_unit_id(command.business_unit_id.clone())
                    .build(),
            ),
            false => None,
        }
    }

    async fn handle<
        C: AsRef<perroute_storage::active_record::datasource::Connection>,
    >(
        &self,
        ctx: &CommandBusContext<'_, C, Self::Command>,
    ) -> CommandHandlerResult<Self::Output> {
        let bu = BusinessUnit::fetch_optional(
            ctx.datasource(),
            BusinessUnitQuery::ById(&ctx.command().business_unit_id),
        )
        .await
        .tap_err(|e| log::error!("Failed to retrieve bu: {e}"))?
        .ok_or(DeleteBusinessUnitCommandError::BusinessUnitNotFound)?;

        if Channel::exists(
            ctx.datasource(),
            ChannelQuery::ByBusinessUnitId(bu.id()),
        )
        .await
        .tap_err(|e| log::error!("Failed to check channel existense: {e}"))?
        {
            return Err(
                DeleteBusinessUnitCommandError::BusinessUnitHasChannels.into(),
            );
        }

        bu.destroy(ctx.datasource())
            .await
            .tap_err(|e| log::error!("Failed to delete business unit: {e}"))
            .map_err(CommandBusError::from)
    }
}
