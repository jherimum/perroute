use crate::command_bus::{
    bus::CommandHandler, commands::UpdateChannelCommand, error::CommandBusError,
};
use async_trait::async_trait;
use derive_new::new;
use perroute_commons::types::id::Id;
use tap::TapFallible;

use super::retrieve_channel;

#[derive(Debug, new)]
pub struct UpdateChannelCommandHandler;

#[derive(thiserror::Error, Debug, Clone)]
pub enum UpdateChannelError {
    #[error("Channel with id {0} nor found")]
    ChannelNotFound(Id),
}

#[async_trait]
impl CommandHandler for UpdateChannelCommandHandler {
    type Command = UpdateChannelCommand;

    #[tracing::instrument(skip(self))]
    async fn handle<'ctx>(
        &self,
        ctx: &mut crate::command_bus::bus::CommandBusContext<'ctx>,
        command: Self::Command,
    ) -> Result<(), CommandBusError> {
        let mut channel = retrieve_channel(ctx, command.channel_id(), |id| {
            UpdateChannelError::ChannelNotFound(id).into()
        })
        .await?;

        channel.set_name(command.name().to_owned());

        channel.update(ctx.tx()).await.tap_err(|e| {
            tracing::error!("Error while updating channel {}: {e}", command.channel_id())
        })?;

        Ok(())
    }
}
