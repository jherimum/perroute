use crate::{
    command_bus::{
        bus::CommandBusContext, commands::CommandType, error::CommandBusError,
        handlers::CommandHandler,
    },
    impl_command, into_event,
};
use anyhow::Context;
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{
    actor::Actor,
    id::Id,
    properties::{Properties, PropertiesError},
};
use perroute_storage::{
    models::{
        channel::{Channel, ChannelQuery},
        route::{Route, RouteBuilder},
        schema::{Schema, SchemasQuery},
    },
    query::FetchableModel,
};
use serde::Serialize;
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Channel with id {0} not found")]
    ChannelNotFound(Id),

    #[error("Schema with id {0} not found")]
    SchemaNotFound(Id),

    #[error("Invalid properties: {0}")]
    InvalidProperties(#[from] PropertiesError),
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct CreateRouteCommand {
    id: Id,
    channel_id: Id,
    schema_id: Id,
    properties: Properties,
}

impl_command!(CreateRouteCommand, CommandType::CreateRoute);
into_event!(CreateRouteCommand);

#[derive(Debug)]
pub struct CreateRouteCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for CreateRouteCommandHandler {
    type Command = CreateRouteCommand;
    type Output = Route;

    #[tracing::instrument(name = "create_route_handler", skip(self, ctx))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        _: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        let channel = Channel::find(ctx.pool(), ChannelQuery::with_id(cmd.channel_id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve channel {}: {e}", cmd.channel_id))?
            .ok_or(Error::ChannelNotFound(cmd.channel_id))?;

        let schema = Schema::find(
            ctx.pool(),
            SchemasQuery::with_id_and_business_unit(cmd.schema_id, *channel.business_unit_id()),
        )
        .await
        .tap_err(|e| tracing::error!("Failed to retrieve schema {}: {e}", cmd.schema_id))?
        .ok_or(Error::SchemaNotFound(cmd.schema_id))?;

        let conn = channel
            .connection(ctx.pool())
            .await
            .context("Connection expected to be found")?;

        let plugin = ctx
            .plugins()
            .get(conn.plugin_id())
            .context("Connector Plugin expected to be found")?;

        let disp = plugin
            .dispatcher(channel.dispatch_type())
            .context("Dispatcher plugin expected to be found")?;

        let props = channel.properties().merge(&cmd.properties);
        disp.configuration().validate(&props).map_err(Error::from)?;

        Ok(RouteBuilder::default()
            .id(cmd.id)
            .channel_id(*channel.id())
            .message_type_id(*schema.message_type_id())
            .business_unit_id(*channel.business_unit_id())
            .schema_id(*schema.id())
            .properties(props)
            .build()
            .context("Failed to build route")?
            .save(ctx.tx())
            .await
            .tap_err(|e| tracing::error!("Failed to save route: {e}"))?)
    }
}
