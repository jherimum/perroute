use crate::bus::Ctx;
use crate::command::Command;
use crate::error::CommandBusError;
use anyhow::anyhow;
use anyhow::Context;
use perroute_commons::types::command_type::CommandType;
use perroute_commons::types::{
    actor::Actor,
    id::Id,
    priority::Priority,
    properties::{Properties, PropertiesError},
};
use perroute_storage::{
    models::route::{Route, RouteQuery},
    query::FetchableModel,
};
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum UpdateRouteError {
    #[error("Route with id {0} not found")]
    RouteNotFound(Id),

    #[error("Invalid properties: {0}")]
    InvalidProperties(#[from] PropertiesError),
}

#[derive(Debug, derive_builder::Builder)]
pub struct UpdateRouteCommand {
    id: Id,
    properties: Option<Properties>,
    priority: Option<Priority>,
}

#[async_trait::async_trait]
impl Command for UpdateRouteCommand {
    type Output = Route;

    #[tracing::instrument(name = "update_route_handler", skip(self, ctx))]
    async fn handle<'tx>(&self, ctx: &mut Ctx<'tx>) -> Result<Self::Output, CommandBusError> {
        let mut route = Route::find(ctx.pool(), RouteQuery::with_id(self.id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve route {}: {e}", self.id))?
            .ok_or(UpdateRouteError::RouteNotFound(self.id))?;

        if self.properties.is_none() && self.priority.is_none() {
            return Ok(route);
        }

        if let Some(props) = self.properties.clone() {
            let channel = route
                .channel(ctx.pool())
                .await
                .context("Channel expected to exists")?;

            let conn = route
                .connection(ctx.pool())
                .await
                .context("Connection expected to exists")?;

            let plugin = ctx
                .plugins()
                .get(conn.plugin_id())
                .ok_or(anyhow!("Plugin {} not found", conn.plugin_id()))?;

            let dispatcher = plugin.dispatcher(channel.dispatch_type()).ok_or(anyhow!(
                "Dispatcher not found for channel dispatch type {}",
                channel.dispatch_type()
            ))?;

            dispatcher
                .configuration()
                .validate(&channel.properties().merge(&props))
                .map_err(UpdateRouteError::from)?;

            route = route.set_properties(props);
        }

        if let Some(priority) = self.priority {
            route = route.set_priority(priority);
        }

        Ok(route
            .update(ctx.pool())
            .await
            .tap_err(|e| tracing::error!("Failed to update route:{e}"))?)
    }

    fn command_type(&self) -> CommandType {
        CommandType::UpdateRoute
    }

    fn supports(&self, actor: &Actor) -> bool {
        true
    }
}
