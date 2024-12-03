use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::Command,
};
use bon::Builder;
use perroute_commons::{events::ChannelDeletedEvent, types::id::Id};
use perroute_storage::repository::{
    channels::{ChannelQuery, ChannelRepository},
    TransactedRepository,
};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum DeleteChannelCommandError {}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct DeleteChannelCommand {
    id: Id,
}

impl Command for DeleteChannelCommand {
    fn event_type(&self) -> perroute_commons::events::EventType {
        perroute_commons::events::EventType::ChannelDeleted
    }

    fn entity_id(&self) -> &Id {
        &self.id
    }
}

pub struct DeleteChannelCommandHandler;

impl CommandHandler for DeleteChannelCommandHandler {
    type Command = DeleteChannelCommand;
    type Output = ();
    type ApplicationEvent = ChannelDeletedEvent;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &crate::commands::CommandWrapper<'_, Self::Command>,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output, Self::ApplicationEvent> {
        let result =
            ChannelRepository::delete(ctx.repository(), &ChannelQuery::ById(&cmd.inner().id))
                .await?;

        //Ok(())

        todo!()
    }
}
