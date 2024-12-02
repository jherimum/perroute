use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::Command,
    CommandBusError,
};
use bon::Builder;
use perroute_commons::types::{id::Id, name::Name, Configuration};
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
    fn event_type(&self) -> perroute_commons::events::EventType {
        perroute_commons::events::EventType::ChannelUpdated
    }

    fn entity_id(&self) -> &Id {
        &self.id
    }
}

pub struct UpdateChannelCommandHandler;

impl CommandHandler for UpdateChannelCommandHandler {
    type Command = UpdateChannelCommand;
    type Output = Channel;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &crate::commands::CommandWrapper<'_, Self::Command>,
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
