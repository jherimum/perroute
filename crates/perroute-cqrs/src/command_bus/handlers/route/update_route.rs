use crate::{
    command_bus::{
        bus::CommandBusContext, commands::CommandType, handlers::CommandHandler, Result,
    },
    impl_command, into_event,
};
use anyhow::anyhow;
use anyhow::Context;
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{
    actor::Actor,
    id::Id,
    properties::{Properties, PropertiesError},
};
use perroute_storage::{
    models::route::{Route, RouteQuery},
    query::FetchableModel,
};
use serde::Serialize;
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum UpdateRouteError {
    #[error("Route with id {0} not found")]
    RouteNotFound(Id),

    #[error("Invalid properties: {0}")]
    InvalidProperties(#[from] PropertiesError),
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct UpdateRouteCommand {
    id: Id,
    properties: Option<Properties>,
}

impl_command!(UpdateRouteCommand, CommandType::UpdateRoute);
into_event!(UpdateRouteCommand);

#[derive(Debug)]
pub struct UpdateRouteCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for UpdateRouteCommandHandler {
    type Command = UpdateRouteCommand;
    type Output = Route;

    #[tracing::instrument(name = "update_route_handler", skip(self, ctx))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        _: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output> {
        let mut route = Route::find(ctx.pool(), RouteQuery::with_id(cmd.id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve route {}: {e}", cmd.id))?
            .ok_or(UpdateRouteError::RouteNotFound(cmd.id))?;

        if cmd.properties.is_none() {
            return Ok(route);
        }

        if let Some(props) = cmd.properties {
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
        Ok(route
            .update(ctx.tx())
            .await
            .tap_err(|e| tracing::error!("Failed to update route:{e}"))?)
    }
}
