use crate::{bus::Ctx, command::Command, error::CommandBusError};
use perroute_commons::types::{actor::Actor, command_type::CommandType, id::Id};
use perroute_storage::{
    models::route::{Route, RouteQuery},
    query::FetchableModel,
};
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum DeleteRouteError {
    #[error("Route with id {0} not found")]
    RouteNotFound(Id),

    #[error("Route {0} could not be deleted: {1}")]
    RouteDelete(Id, &'static str),
}

#[derive(Debug, derive_builder::Builder)]
pub struct DeleteRouteCommand {
    id: Id,
}

#[async_trait::async_trait]
impl Command for DeleteRouteCommand {
    type Output = bool;

    #[tracing::instrument(name = "delete_route_handler", skip(self, ctx))]
    async fn handle<'tx>(&self, ctx: &mut Ctx<'tx>) -> Result<Self::Output, CommandBusError> {
        let route = Route::find(ctx.pool(), RouteQuery::with_id(self.id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve route {}: {e}", self.id))?
            .ok_or(DeleteRouteError::RouteNotFound(self.id))?;

        Ok(route
            .delete(ctx.pool())
            .await
            .tap_err(|e| tracing::error!("Failed to delete route {}: {e}", self.id))?)
    }

    fn command_type(&self) -> CommandType {
        CommandType::DeleteRoute
    }

    fn supports(&self, actor: &Actor) -> bool {
        true
    }
}
