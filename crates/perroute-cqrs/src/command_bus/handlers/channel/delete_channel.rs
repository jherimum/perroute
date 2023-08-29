use crate::{
    command_bus::{
        bus::CommandBusContext, commands::CommandType, handlers::CommandHandler, Result,
    },
    impl_command, into_event,
};
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{actor::Actor, id::Id};
use perroute_storage::{
    models::{
        channel::{Channel, ChannelQuery},
        route::Route,
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
pub enum Error {
    #[error("Channel {0} not found")]
    ChannelNotFound(Id),
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
    ) -> Result<Self::Output> {
        let channel = Channel::find(ctx.pool(), ChannelQuery::with_id(cmd.id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve channel {}: {e}", cmd.id))?
            .ok_or(Error::ChannelNotFound(cmd.id))?;

        Route::delete_by_channel(ctx.tx(), channel.id()).await?;

        Ok(channel
            .delete(ctx.tx())
            .await
            .tap_err(|e| tracing::error!("Failed to delete channel{}: {e}", cmd.id))?)
    }
}
