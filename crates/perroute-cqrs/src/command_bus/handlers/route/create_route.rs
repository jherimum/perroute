use crate::{
    command_bus::{
        bus::CommandBusContext, commands::CommandType, handlers::CommandHandler, Result,
    },
    impl_command, into_event,
};
use anyhow::Context;
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{
    id::Id,
    priority::Priority,
    properties::{Properties, PropertiesError},
};
use perroute_storage::{
    models::{
        channel::{Channel, ChannelQuery},
        message_type::{MessageType, MessageTypeQuery},
        route::{Route, RouteBuilder},
    },
    query::FetchableModel,
};
use serde::Serialize;
use sqlx::PgPool;
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum CreateRouteError {
    #[error("Channel with id {0} not found")]
    ChannelNotFound(Id),

    #[error("Message type with id {0} not found")]
    MessageTypeNotFound(Id),

    #[error("Invalid properties: {0}")]
    InvalidProperties(#[from] PropertiesError),
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct CreateRouteCommand {
    id: Id,
    channel_id: Id,
    message_type_id: Id,
    properties: Properties,
    priority: Priority,
}

impl_command!(CreateRouteCommand, CommandType::CreateRoute);
into_event!(CreateRouteCommand);

#[derive(Debug, derive_getters::Getters)]
pub struct CreateRouteCommandHandler {
    pool: PgPool,
}

impl CreateRouteCommandHandler {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl CommandHandler for CreateRouteCommandHandler {
    type Command = CreateRouteCommand;
    type Output = Route;

    #[tracing::instrument(name = "create_route_handler", skip(self, ctx))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext,

        cmd: Self::Command,
    ) -> Result<Self::Output> {
        let channel = Channel::find(ctx.pool(), ChannelQuery::with_id(cmd.channel_id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve channel {}: {e}", cmd.channel_id))?
            .ok_or(CreateRouteError::ChannelNotFound(cmd.channel_id))?;

        let message_type = MessageType::find(ctx.pool(), MessageTypeQuery::with_id(cmd.id))
            .await
            .tap_err(|e| {
                tracing::error!(
                    "Failed to retrieve message type{}: {e}",
                    cmd.message_type_id
                )
            })?
            .ok_or(CreateRouteError::MessageTypeNotFound(cmd.message_type_id))?;

        let conn = channel.connection(ctx.pool()).await?;

        let plugin = ctx
            .plugins()
            .get(conn.plugin_id())
            .context("Connector Plugin expected to be found")?;

        let disp = plugin
            .dispatcher(channel.dispatch_type())
            .context("Dispatcher plugin expected to be found")?;

        let props = channel.properties().merge(&cmd.properties);
        disp.configuration()
            .validate(&props)
            .map_err(CreateRouteError::from)?;

        Ok(RouteBuilder::default()
            .id(cmd.id)
            .channel_id(*channel.id())
            .message_type_id(*message_type.id())
            .business_unit_id(*channel.business_unit_id())
            .properties(props)
            .priority(cmd.priority)
            .build()
            .context("Failed to build route")?
            .save(ctx.pool())
            .await
            .tap_err(|e| tracing::error!("Failed to save route: {e}"))?)
    }
}
