use crate::{bus::Ctx, command::Command, error::CommandBusError};
use anyhow::Context;
use perroute_commons::types::{
    actor::Actor,
    command_type::CommandType,
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

#[derive(Debug, derive_builder::Builder)]
pub struct CreateRouteCommand {
    id: Id,
    channel_id: Id,
    message_type_id: Id,
    properties: Properties,
    priority: Priority,
}

#[async_trait::async_trait]
impl Command for CreateRouteCommand {
    type Output = Route;

    #[tracing::instrument(name = "create_route_handler", skip(self, ctx))]
    async fn handle<'tx>(&self, ctx: &mut Ctx<'tx>) -> Result<Self::Output, CommandBusError> {
        let channel = Channel::find(ctx.pool(), ChannelQuery::with_id(self.channel_id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve channel {}: {e}", self.channel_id))?
            .ok_or(CreateRouteError::ChannelNotFound(self.channel_id))?;

        let message_type = MessageType::find(ctx.pool(), MessageTypeQuery::with_id(self.id))
            .await
            .tap_err(|e| {
                tracing::error!(
                    "Failed to retrieve message type{}: {e}",
                    self.message_type_id
                )
            })?
            .ok_or(CreateRouteError::MessageTypeNotFound(self.message_type_id))?;

        let conn = channel.connection(ctx.pool()).await?;

        let plugin = ctx
            .plugins()
            .get(conn.plugin_id())
            .context("Connector Plugin expected to be found")?;

        let disp = plugin
            .dispatcher(channel.dispatch_type())
            .context("Dispatcher plugin expected to be found")?;

        let props = channel.properties().merge(&self.properties);
        disp.configuration()
            .validate(&props)
            .map_err(CreateRouteError::from)?;

        Ok(RouteBuilder::default()
            .id(self.id)
            .channel_id(*channel.id())
            .message_type_id(*message_type.id())
            .business_unit_id(*channel.business_unit_id())
            .properties(props)
            .priority(self.priority)
            .build()
            .context("Failed to build route")?
            .save(ctx.pool())
            .await
            .tap_err(|e| tracing::error!("Failed to save route: {e}"))?)
    }

    fn command_type(&self) -> CommandType {
        CommandType::CreateRoute
    }

    fn supports(&self, actor: &Actor) -> bool {
        true
    }
}
