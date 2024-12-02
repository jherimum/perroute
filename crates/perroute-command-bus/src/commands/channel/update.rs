use crate::{
    bus::{Command, CommandBusContext, CommandHandler, CommandHandlerResult, CommandWrapper},
    CommandBusError,
};
use bon::Builder;
use perroute_commons::{
    commands::CommandType,
    types::{id::Id, name::Name, Configuration},
};
use perroute_storage::{
    models::channel::Channel,
    repository::{
        channels::{ChannelQuery, ChannelRepository},
        TransactedRepository,
    },
};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum UpdateChannelCommandError {
    #[error("Channel not found")]
    NotFound,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct UpdateChannelCommand {
    id: Id,
    name: Name,
    configuration: Configuration,
    enabled: bool,
}

impl Command for UpdateChannelCommand {
    type Output = Channel;

    fn command_type(&self) -> CommandType {
        CommandType::UpdateChannel
    }

    fn to_event(
        &self,
        created_at: &perroute_commons::types::Timestamp,
        actor: &perroute_commons::types::actor::Actor,
        output: &Self::Output,
    ) -> perroute_commons::events::Event {
        todo!()
    }
}

pub struct UpdateChannelCommandHandler;

impl CommandHandler for UpdateChannelCommandHandler {
    type Command = UpdateChannelCommand;
    type Output = Channel;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: CommandWrapper<'_, Self::Command>,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        let channel =
            ChannelRepository::find(ctx.repository(), &ChannelQuery::ById(&cmd.inner().id))
                .await?
                .ok_or(CommandBusError::from(UpdateChannelCommandError::NotFound))?
                .set_configuration(cmd.inner().configuration.clone())
                .set_enabled(cmd.inner().enabled)
                .set_name(cmd.inner().name.clone())
                .set_updated_at(cmd.created_at().clone());

        let channel = ChannelRepository::update(ctx.repository(), channel).await?;

        Ok(channel)
    }
}
