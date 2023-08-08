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
    models::channel::{Channel, ChannelsQueryBuilder},
    query::FetchableModel,
};

command!(
    DeleteChannelCommand,
    CommandType::DeleteChannel,
    channel_id: Id
);
into_event!(DeleteChannelCommand);

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

    #[tracing::instrument(name = "delete_channel_handler", skip(self, ctx))]
    async fn handle<'ctx>(
        &self,
        ctx: &mut CommandBusContext<'ctx>,
        actor: &Actor,
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

        // if let Some(channel) = channel {
        //     let message_types = MessageType::count(
        //         ctx.pool(),
        //         MessageTypeQueryBuilder::default()
        //             .channel_id(Some(*channel.id()))
        //             .build()
        //             .unwrap(),
        //     )
        //     .await?;

        //     if message_types == 0 {
        //         channel
        //             .delete(ctx.tx())
        //             .await
        //             .tap_err(|e| {
        //                 tracing::error!("Failed to delete channel {}: {e}", command.channel_id());
        //             })
        //             .map_err(CommandBusError::from)
        //     } else {
        //         Err(CommandBusError::ExpectedError(
        //             "There are message types registered for this channel",
        //         ))
        //     }
        // } else {
        //     Ok(false)
        // }
        Ok(true)
    }
}
