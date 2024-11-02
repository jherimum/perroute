use crate::bus::{Command, CommandBusContext, CommandHandler, CommandHandlerResult};
use bon::Builder;
use perroute_commons::{commands::CommandType, types::id::Id};
use perroute_storage::repository::TransactedRepository;

#[derive(Debug, thiserror::Error)]
pub enum DeleteRouteCommandError {}

#[derive(Debug, Clone, Builder)]
pub struct DeleteRouteCommand {
    id: Id,
}

impl Command for DeleteRouteCommand {
    fn command_type(&self) -> CommandType {
        CommandType::DeleteRoute
    }
}

pub struct DeleteRouteCommandHandler;

impl CommandHandler for DeleteRouteCommandHandler {
    type Command = DeleteRouteCommand;
    type Output = bool;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        todo!()
    }
}
