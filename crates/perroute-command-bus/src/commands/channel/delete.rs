use crate::{
    bus::{Command, CommandBusContext, CommandHandler, CommandHandlerOutput, CommandHandlerResult},
    commands::CommandType,
};
use bon::Builder;
use perroute_commons::types::id::Id;
use perroute_events::event::Event;
use perroute_storage::repository::{
    channels::{ChannelQuery, ChannelRepository},
    TransactedRepository,
};

#[derive(Debug, thiserror::Error)]
pub enum DeleteChannelCommandError {}

#[derive(Debug, Clone, Builder)]
pub struct DeleteChannelCommand {
    id: Id,
}

impl Command for DeleteChannelCommand {
    fn command_type(&self) -> CommandType {
        CommandType::DeleteChannel
    }
}

pub struct DeleteChannelCommandHandler;

impl CommandHandler for DeleteChannelCommandHandler {
    type Command = DeleteChannelCommand;
    type Output = bool;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        let result =
            ChannelRepository::delete(ctx.repository(), &ChannelQuery::ById(cmd.id.clone()))
                .await?;

        CommandHandlerOutput::new(result > 0)
            .with_event(Event::ChannelDeleted(cmd.id.clone()))
            .ok()
    }
}
