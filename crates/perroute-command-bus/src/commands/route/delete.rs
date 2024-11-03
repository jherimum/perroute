use crate::bus::{
    Command, CommandBusContext, CommandHandler, CommandHandlerOutput, CommandHandlerResult,
};
use bon::Builder;
use perroute_commons::{commands::CommandType, events::Event, types::id::Id};
use perroute_storage::repository::{
    routes::{RouteQuery, RouteRepository},
    TransactedRepository,
};

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
        let deleted =
            RouteRepository::delete(ctx.repository(), &RouteQuery::ById(&cmd.id)).await? > 0;

        CommandHandlerOutput::new(deleted)
            .with_event(Event::RouteDeleted(cmd.id.clone()))
            .ok()
    }
}
