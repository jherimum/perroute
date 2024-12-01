use crate::bus::{
    Command, CommandBusContext, CommandHandler, CommandHandlerOutput, CommandHandlerResult,
};
use bon::Builder;
use perroute_commons::{commands::CommandType, types::id::Id};
use perroute_storage::repository::{
    routes::{RouteQuery, RouteRepository},
    TransactedRepository,
};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum DeleteRouteCommandError {}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct DeleteRouteCommand {
    id: Id,
}

impl Command for DeleteRouteCommand {
    fn command_type(&self) -> CommandType {
        CommandType::DeleteRoute
    }

    fn to_event<R: TransactedRepository>(
        &self,
        ctx: &CommandBusContext<'_, R>,
    ) -> perroute_commons::events::Event {
        todo!()
    }
}

pub struct DeleteRouteCommandHandler;

impl CommandHandler for DeleteRouteCommandHandler {
    type Command = DeleteRouteCommand;
    type Output = bool;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        let deleted =
            RouteRepository::delete(ctx.repository(), &RouteQuery::ById(&cmd.id)).await? > 0;

        CommandHandlerOutput::new(deleted).ok()
    }
}
