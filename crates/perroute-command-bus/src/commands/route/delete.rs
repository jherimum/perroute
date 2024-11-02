use crate::{
    bus::{Command, CommandBusContext, CommandHandler, CommandHandlerResult},
    CommandBusResult,
};
use bon::Builder;
use perroute_commons::types::id::Id;
use perroute_storage::repository::TransactedRepository;

#[derive(Debug, thiserror::Error)]
pub enum DeleteRouteCommandError {}

#[derive(Debug, Clone, Builder)]
pub struct DeleteRouteCommand {
    id: Id,
}

impl Command for DeleteRouteCommand {}

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
