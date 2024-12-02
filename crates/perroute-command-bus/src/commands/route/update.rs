use crate::bus::{
    Command, CommandBusContext, CommandHandler, CommandHandlerResult, CommandWrapper,
};
use bon::Builder;
use perroute_commons::{
    commands::CommandType,
    types::{id::Id, priority::Priority, Configuration},
};
use perroute_storage::{
    models::route::Route,
    repository::{
        routes::{RouteQuery, RouteRepository},
        TransactedRepository,
    },
};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum UpdateRouteCommandError {
    #[error("Route not found")]
    NotFound,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct UpdateRouteCommand {
    id: Id,
    configuration: Configuration,
    priority: Priority,
    enabled: bool,
}

impl Command for UpdateRouteCommand {
    type Output = Route;

    fn command_type(&self) -> CommandType {
        CommandType::UpdateRoute
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

pub struct UpdateRouteCommandHandler;

impl CommandHandler for UpdateRouteCommandHandler {
    type Command = UpdateRouteCommand;
    type Output = Route;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: CommandWrapper<'_, Self::Command>,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        let route = RouteRepository::get(ctx.repository(), &RouteQuery::ById(&cmd.inner().id))
            .await?
            .ok_or(UpdateRouteCommandError::NotFound)?
            .set_configuration(cmd.inner().configuration.clone())
            .set_enabled(cmd.inner().enabled)
            .set_priority(cmd.inner().priority.clone())
            .set_updated_at(cmd.created_at().clone());

        let route = RouteRepository::update(ctx.repository(), route).await?;

        Ok(route)
    }
}
