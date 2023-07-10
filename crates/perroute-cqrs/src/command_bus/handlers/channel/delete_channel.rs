use crate::command_bus::{
    bus::CommandBusContext, commands::DeleteChannelCommand, error::CommandBusError,
    handlers::CommandHandler,
};
use async_trait::async_trait;
use perroute_commons::types::id::Id;
use perroute_storage::models::{
    channel::{Channel, ChannelsQueryBuilder},
    message_type::MessageType,
};
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
    type Output = bool;

    #[tracing::instrument(skip(self))]
    async fn handle<'ctx, 'a>(
        &self,
        ctx: &mut CommandBusContext<'ctx, 'a>,
        command: Self::Command,
    ) -> Result<bool, CommandBusError> {
        let channel = Channel::find(
            ctx.tx(),
            ChannelsQueryBuilder::default()
                .id(Some(*command.channel_id()))
                .build()
                .unwrap(),
        )
        .await?;

        if let Some(channel) = channel {
            let message_types =
                MessageType::find_by_channel(ctx.pool(), *command.channel_id()).await?;

            if message_types.is_empty() {
                channel
                    .delete(ctx.tx())
                    .await
                    .tap_err(|e| {
                        tracing::error!("Failed to delete channel {}: {e}", command.channel_id())
                    })
                    .map_err(CommandBusError::from)
            } else {
                Err(CommandBusError::ExpectedError(
                    "There are message types registered for this channel",
                ))
            }
        } else {
            Ok(false)
        }
    }
}
