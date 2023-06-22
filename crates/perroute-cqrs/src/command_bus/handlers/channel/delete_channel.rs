use super::retrieve_channel;
use crate::command_bus::{
    bus::{CommandBusContext, CommandHandler},
    commands::DeleteChannelCommand,
    error::CommandBusError,
};
use async_trait::async_trait;
use perroute_commons::types::id::Id;
use tap::TapFallible;

#[derive(Debug)]
pub struct DeleteChannelCommandHandler;

#[derive(thiserror::Error, Debug, Clone)]
pub enum DeleteChannelError {
    #[error("Channel with id {0} nor found")]
    ChannelNotFound(Id),
}

#[async_trait]
impl CommandHandler for DeleteChannelCommandHandler {
    type Command = DeleteChannelCommand;

    #[tracing::instrument(skip(self))]
    async fn handle<'ctx>(
        &self,
        ctx: &mut CommandBusContext<'ctx>,
        command: Self::Command,
    ) -> Result<(), CommandBusError> {
        retrieve_channel(ctx, command.channel_id(), |id| {
            DeleteChannelError::ChannelNotFound(id).into()
        })
        .await?
        .delete(ctx.tx())
        .await
        .tap_err(|e| tracing::error!("Failed to delete channel {}: {e}", command.channel_id()))
        .map_err(CommandBusError::from)
        .map(|_| ())
    }
}
