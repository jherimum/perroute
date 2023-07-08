use crate::command_bus::{
    bus::CommandBusContext, commands::UpdateChannelCommand, error::CommandBusError,
    handlers::CommandHandler,
};
use async_trait::async_trait;
use derive_new::new;
use perroute_commons::types::id::Id;
use perroute_storage::models::channel::Channel;
use tap::TapFallible;

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
    type Output = Channel;

    #[tracing::instrument(skip(self))]
    async fn handle<'ctx, 'a>(
        &self,
        ctx: &mut CommandBusContext<'ctx, 'a>,
        command: Self::Command,
    ) -> Result<Channel, CommandBusError> {
        let channel = Channel::find_by_id(ctx.tx(), *command.channel_id()).await?;

        if let Some(channel) = channel {
            channel
                .set_name(command.name().to_owned())
                .update(ctx.tx())
                .await
                .tap_err(|e| {
                    tracing::error!("Error while updating channel {}: {e}", command.channel_id())
                })
                .map_err(CommandBusError::from)
        } else {
            tracing::error!("Channel {} not found", command.channel_id());
            Err(CommandBusError::UnexpectedError("Channel do not exists"))
        }
    }
}
