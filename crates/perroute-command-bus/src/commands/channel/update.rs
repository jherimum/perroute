use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::Command,
    impl_command, CommandBusError,
};
use perroute_commons::{
    events::ChannelUpdatedEvent,
    types::{id::Id, name::Name, Configuration},
};
use perroute_storage::{
    models::channel::Channel,
    repository::{
        channels::{ChannelQuery, ChannelRepository},
        TransactedRepository,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum UpdateChannelCommandError {
    #[error("Channel not found")]
    NotFound,
}

impl_command!(UpdateChannelCommand, {
    channel_id: Id,
    name: Name,
    configuration: Configuration,
    enabled: bool
});

pub struct UpdateChannelCommandHandler;

impl CommandHandler for UpdateChannelCommandHandler {
    type Command = UpdateChannelCommand;
    type Output = Channel;
    type ApplicationEvent = ChannelUpdatedEvent;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &crate::commands::CommandWrapper<'_, Self::Command>,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output, Self::ApplicationEvent> {
        let channel = ChannelRepository::find(
            ctx.repository(),
            &ChannelQuery::ById(&cmd.inner().channel_id),
        )
        .await?
        .ok_or(CommandBusError::from(UpdateChannelCommandError::NotFound))?
        .set_configuration(cmd.inner().configuration.clone())
        .set_enabled(cmd.inner().enabled)
        .set_name(cmd.inner().name.clone())
        .set_updated_at(cmd.created_at().clone());

        let channel = ChannelRepository::update(ctx.repository(), channel).await?;

        //Ok(channel)
        todo!()
    }
}
