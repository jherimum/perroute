use crate::bus::{
    Command, CommandBusContext, CommandHandler, CommandHandlerResult, CommandWrapper,
};
use bon::Builder;
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
    type Output = ();

    fn command_type(&self) -> CommandType {
        CommandType::DeleteChannel
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

pub struct DeleteChannelCommandHandler;

impl CommandHandler for DeleteChannelCommandHandler {
    type Command = DeleteChannelCommand;
    type Output = ();

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: CommandWrapper<'_, Self::Command>,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        let result =
            ChannelRepository::delete(ctx.repository(), &ChannelQuery::ById(&cmd.inner().id))
                .await?;

        Ok(())
    }
}
