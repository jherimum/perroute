use crate::bus::{
    Command, CommandBusContext, CommandHandler, CommandHandlerOutput, CommandHandlerResult,
};
use bon::Builder;
use perroute_commons::{
    commands::CommandType,
    events::Event,
    types::{id::Id, priority::Priority, Configuration, Timestamp},
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
    business_unit_id: Id,
    configuration: Configuration,
    priority: Priority,
    enabled: bool,
}

impl Command for UpdateRouteCommand {
    fn command_type(&self) -> CommandType {
        CommandType::UpdateRoute
    }
}

pub struct UpdateRouteCommandHandler;

impl CommandHandler for UpdateRouteCommandHandler {
    type Command = UpdateRouteCommand;
    type Output = Route;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        let route = RouteRepository::get(ctx.repository(), &RouteQuery::ById(&cmd.id))
            .await?
            .ok_or(UpdateRouteCommandError::NotFound)?
            .set_configuration(cmd.configuration.clone())
            .set_enabled(cmd.enabled)
            .set_priority(cmd.priority.clone())
            .set_updated_at(Timestamp::now());

        let route = RouteRepository::update(ctx.repository(), route).await?;

        CommandHandlerOutput::new(route)
            .with_event(Event::RouteUpdated(cmd.id.clone()))
            .ok()
    }
}
