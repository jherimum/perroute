use crate::{
    command_bus::{
        bus::CommandBusContext, commands::CommandType, handlers::CommandHandler, Result,
    },
    impl_command, into_event,
};
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{actor::Actor, id::Id};
use perroute_storage::{
    models::route::{Route, RouteQuery},
    query::FetchableModel,
};
use serde::Serialize;
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum DeleteRouteError {
    #[error("Route with id {0} not found")]
    RouteNotFound(Id),

    #[error("Route {0} could not be deleted: {1}")]
    RouteDelete(Id, &'static str),
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct DeleteRouteCommand {
    id: Id,
}

impl_command!(DeleteRouteCommand, CommandType::DeleteRoute);
into_event!(DeleteRouteCommand);

#[derive(Debug)]
pub struct DeleteRouteCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for DeleteRouteCommandHandler {
    type Command = DeleteRouteCommand;
    type Output = bool;

    #[tracing::instrument(name = "delete_route_handler", skip(self, ctx))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        _: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output> {
        let route = Route::find(ctx.pool(), RouteQuery::with_id(cmd.id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve route {}: {e}", cmd.id))?
            .ok_or(DeleteRouteError::RouteNotFound(cmd.id))?;

        Ok(route
            .delete(ctx.tx())
            .await
            .tap_err(|e| tracing::error!("Failed to delete route {}: {e}", cmd.id))?)
    }
}
