use crate::bus::{
    Command, CommandBusContext, CommandHandler, CommandHandlerResult, CommandWrapper,
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
    type Output = ();

    fn command_type(&self) -> CommandType {
        CommandType::DeleteRoute
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

pub struct DeleteRouteCommandHandler;

impl CommandHandler for DeleteRouteCommandHandler {
    type Command = DeleteRouteCommand;
    type Output = ();

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: CommandWrapper<'_, Self::Command>,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        let deleted = RouteRepository::delete(ctx.repository(), &RouteQuery::ById(&cmd.inner().id))
            .await?
            > 0;

        Ok(())
    }
}
