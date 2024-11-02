use crate::bus::{
    Command, CommandBusContext, CommandHandler, CommandHandlerOutput, CommandHandlerResult,
};
use bon::Builder;
use perroute_commons::events::Event;
use perroute_commons::{commands::CommandType, types::id::Id};
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
