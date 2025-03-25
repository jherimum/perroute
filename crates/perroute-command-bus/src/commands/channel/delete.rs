use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::Command,
    impl_command, CommandBusError,
};
use perroute_commons::{events::ChannelDeletedEvent, types::id::Id};
use perroute_storage::{
    active_record::{
        channel::ChannelQuery, datasource::Connection, ActiveRecord,
    },
    models::channel::Channel,
};
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum DeleteChannelCommandError {
    #[error("Channel not found")]
    ChannelNotFound(Id),
}

impl_command!(DeleteChannelCommand, {
    channel_id: Id
});

pub struct DeleteChannelCommandHandler;

impl CommandHandler for DeleteChannelCommandHandler {
    type Command = DeleteChannelCommand;
    type Output = bool;
    type E = ChannelDeletedEvent;

    async fn handle<C: AsRef<Connection>>(
        &self,
        ctx: &CommandBusContext<'_, C, Self::Command>,
    ) -> CommandHandlerResult<Self::Output> {
        let channel = Channel::fetch_optional(
            ctx.datasource(),
            ChannelQuery::ById(&ctx.command().channel_id),
        )
        .await?
        .ok_or(DeleteChannelCommandError::ChannelNotFound(
            ctx.command().channel_id.clone(),
        ))?;

        channel
            .destroy(ctx.datasource())
            .await
            .tap_err(|e| log::error!("Failed to delete channel: {e}"))
            .map_err(CommandBusError::from)
    }
}
