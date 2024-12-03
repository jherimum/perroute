use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::Command,
    impl_command,
};
use perroute_commons::{
    events::RouteUpdatedEvent,
    types::{id::Id, priority::Priority, Configuration},
};
use perroute_storage::{
    models::route::Route,
    repository::{
        routes::{RouteQuery, RouteRepository},
        TransactedRepository,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum UpdateRouteCommandError {
    #[error("Route not found")]
    NotFound,
}

impl_command!(UpdateRouteCommand, {
    route_id: Id,
    configuration: Configuration,
    priority: Priority,
    enabled: bool,
});

pub struct UpdateRouteCommandHandler;

impl CommandHandler for UpdateRouteCommandHandler {
    type Command = UpdateRouteCommand;
    type Output = Route;
    type ApplicationEvent = RouteUpdatedEvent;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &crate::commands::CommandWrapper<'_, Self::Command>,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output, Self::ApplicationEvent> {
        let route =
            RouteRepository::get(ctx.repository(), &RouteQuery::ById(&cmd.inner().route_id))
                .await?
                .ok_or(UpdateRouteCommandError::NotFound)?
                .set_configuration(cmd.inner().configuration.clone())
                .set_enabled(cmd.inner().enabled)
                .set_priority(cmd.inner().priority.clone())
                .set_updated_at(cmd.created_at().clone());

        let route = RouteRepository::update(ctx.repository(), route).await?;

        //Ok(route)
        todo!()
    }
}
