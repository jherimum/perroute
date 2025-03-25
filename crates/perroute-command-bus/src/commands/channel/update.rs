use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::Command,
    impl_command,
};
use perroute_commons::{
    events::ChannelUpdatedEvent,
    types::{id::Id, name::Name, Configuration},
};
use perroute_storage::{
    active_record::{channel::ChannelQuery, ActiveRecord},
    models::channel::Channel,
};
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum UpdateChannelCommandError {
    #[error("Channel not found")]
    ChannelNotFound,
}

impl_command!(UpdateChannelCommand, {
    channel_id: Id,
    name: Option<Name>,
    configuration: Option<Configuration>,
    enabled: Option<bool>
});

pub struct UpdateChannelCommandHandler;

impl CommandHandler for UpdateChannelCommandHandler {
    type Command = UpdateChannelCommand;
    type Output = Channel;
    type E = ChannelUpdatedEvent;

    fn into_event(
        command: &Self::Command,
        output: &Self::Output,
    ) -> Option<Self::E> {
        Some(
            ChannelUpdatedEvent::builder()
                .maybe_configuration(command.configuration.clone())
                .maybe_enabled(command.enabled)
                .maybe_name(command.name.clone())
                .id(output.id())
                .build(),
        )
    }

    async fn handle<
        C: AsRef<perroute_storage::active_record::datasource::Connection>,
    >(
        &self,
        ctx: &CommandBusContext<'_, C, Self::Command>,
    ) -> CommandHandlerResult<Self::Output> {
        let mut channel = Channel::fetch_optional(
            ctx.datasource(),
            ChannelQuery::ById(&ctx.command().channel_id),
        )
        .await?
        .ok_or(UpdateChannelCommandError::ChannelNotFound)?;

        if let Some(c) = ctx.command().configuration.as_ref() {
            channel = channel.set_configuration(c);
        }

        if let Some(name) = ctx.command().name.as_ref() {
            channel = channel.set_name(name);
        }

        if let Some(enabled) = ctx.command().enabled {
            channel = channel.set_enabled(enabled);
        }

        channel = channel.set_updated_at(ctx.timestamp());

        Ok(channel
            .update(ctx.datasource())
            .await
            .tap_err(|e| log::error!("Failed to update channel: {e}"))?)
    }
}
