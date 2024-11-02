use crate::{
    bus::{Command, CommandBusContext, CommandHandler, CommandHandlerOutput, CommandHandlerResult},
    CommandBusError,
};
use bon::Builder;
use perroute_commons::{
    commands::CommandType,
    events::Event,
    types::{id::Id, name::Name, Configuration, Timestamp},
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

#[derive(Debug, Clone, Builder)]
pub struct UpdateChannelCommand {
    id: Id,
    name: Name,
    configuration: Configuration,
    enabled: bool,
}

impl Command for UpdateChannelCommand {
    fn command_type(&self) -> CommandType {
        CommandType::UpdateChannel
    }
}

pub struct UpdateChannelCommandHandler;

impl CommandHandler for UpdateChannelCommandHandler {
    type Command = UpdateChannelCommand;
    type Output = Channel;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        let channel =
            ChannelRepository::find(ctx.repository(), &ChannelQuery::ById(cmd.id.clone()))
                .await?
                .ok_or(CommandBusError::from(UpdateChannelCommandError::NotFound))?
                .set_configuration(cmd.configuration.clone())
                .set_enabled(cmd.enabled)
                .set_name(cmd.name.clone())
                .set_updated_at(Timestamp::now());

        let channel = ChannelRepository::update(ctx.repository(), channel).await?;

        CommandHandlerOutput::new(channel.clone())
            .with_event(Event::ChannelUpdated(channel.id().clone()))
            .ok()
    }
}
