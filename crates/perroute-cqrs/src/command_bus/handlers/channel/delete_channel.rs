use crate::{
    command_bus::{
        bus::CommandBusContext, commands::CommandType, error::CommandBusError,
        handlers::CommandHandler,
    },
    impl_command, into_event,
};
use anyhow::Context;
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{actor::Actor, id::Id};
use perroute_storage::{
    models::{
        channel::{Channel, ChannelQuery},
        route::{Route, RouteQueryBuilder},
    },
    query::FetchableModel,
};
use tap::TapFallible;

#[derive(Debug, serde::Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct DeleteChannelCommand {
    id: Id,
}
impl_command!(DeleteChannelCommand, CommandType::DeleteChannel);
into_event!(DeleteChannelCommand);

#[derive(Debug, thiserror::Error)]
pub enum DeleteChannelCommandHandlerError {
    #[error("Channel {0} not found")]
    ChannelNotFound(Id),

    #[error("Channel {0} could not be deleted: {1}")]
    ChannelNotDeletable(Id, &'static str),
}

#[derive(Debug)]
pub struct DeleteChannelCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for DeleteChannelCommandHandler {
    type Command = DeleteChannelCommand;
    type Output = bool;

    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        _: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        let channel = Channel::find(ctx.pool(), ChannelQuery::with_id(cmd.id))
            .await?
            .ok_or(DeleteChannelCommandHandlerError::ChannelNotFound(cmd.id))
            .tap_err(|e| tracing::error!("{e}"))?;

        if Route::exists(
            ctx.pool(),
            RouteQueryBuilder::default()
                .channel_id(Some(*channel.id()))
                .build()
                .context("Failed to build RouteQuery")?,
        )
        .await
        .tap_err(|e| tracing::error!("Failed to check if route exists:{e}"))?
        {
            return Err(DeleteChannelCommandHandlerError::ChannelNotDeletable(
                *channel.id(),
                "There are routes associated to this channel",
            )
            .into());
        };

        Ok(channel
            .delete(ctx.tx())
            .await
            .tap_err(|e| tracing::error!("Failed fo delete channel: {e}"))?)
    }
}
