use async_trait::async_trait;
use chrono::Utc;
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::{
    new_id,
    types::{actor::Actor, id::Id},
};
use perroute_connectors::types::delivery::Delivery;
use perroute_connectors::{
    api::{DispatchError, DispatchRequest, DispatchResponse},
    types::plugin_id::ConnectorPluginId,
};
use perroute_messaging::events::EventType;
use perroute_storage::{
    models::{
        message::{Message, MessageQueryBuilder, Status},
        message_dispatch::{MessageDispatch, MessageDispatchBuilder, MessageDispatchResult},
        route::Route,
    },
    query::FetchableModel,
};
use serde::Serialize;
use sqlx::{types::Json, PgPool};
use tap::{TapFallible, TapOptional};

use crate::{
    command_bus::{
        bus::CommandBusContext, commands::CommandType, error::CommandBusError,
        handlers::CommandHandler,
    },
    impl_command, into_event,
};

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct DistributeMessageCommand {
    message_id: Id,
}
into_event!(
    DistributeMessageCommand,
    EventType::MessageCreated,
    |cmd: &DistributeMessageCommand| { cmd.message_id }
);
impl_command!(DistributeMessageCommand, CommandType::DistributeMessage);

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Message {0} not found")]
    MessageNotFound(Id),

    #[error("Invalid message {0} state")]
    InvalidMessageState(Id),

    #[error("Schema {0} is disabled")]
    SchemaDisabeld(Id),
}

#[derive(Debug)]
pub struct DistributeMessageCommandHandler;

#[async_trait]
impl CommandHandler for DistributeMessageCommandHandler {
    type Command = DistributeMessageCommand;
    type Output = Message;

    #[tracing::instrument(name = "distribute_message_handler", skip(self, ctx))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        actor: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        let message = retrieve_message(ctx.pool(), cmd.message_id).await?;

        if Status::Pending != *message.status() {
            tracing::error!("To be distributed message must be in pending state");
            return Err(Error::InvalidMessageState(cmd.message_id).into());
        }

        let schema = message.schema(ctx.pool()).await?;

        if !*schema.enabled() {
            tracing::warn!("Schema {} is disabled", schema.id());
            return Err(Error::SchemaDisabeld(*schema.id()).into());
        }

        for delivery in message.deliveries().iter() {
            for route in
                Route::dispatch_route_stack(ctx.pool(), schema.id(), &delivery.dispatch_type())
                    .await?
            {
                let conn = route.connection(ctx.pool()).await?;
                let plugin = ctx.plugins().get(conn.plugin_id()).unwrap();
                let dispatcher = plugin.dispatcher(&delivery.dispatch_type()).unwrap();
                let request = build_request();

                let result = dispatcher.dispatch(&request).await;

                let message_dispatch =
                    register_message_dispatch(ctx, &message, &plugin.id(), delivery, &result)
                        .await?;

                result.is_err();
            }
        }

        let message = message
            .set_status(Status::Distributed)
            .update(ctx.tx())
            .await
            .tap_err(|e| tracing::error!("Failed to update message: {e}"))?;

        Ok(message)
    }
}

async fn register_message_dispatch(
    ctx: &mut CommandBusContext<'_>,
    message: &Message,
    plugin_id: &ConnectorPluginId,
    delivery: &Delivery,
    result: &Result<DispatchResponse, DispatchError>,
) -> Result<MessageDispatch, sqlx::Error> {
    MessageDispatchBuilder::default()
        .id(new_id!())
        .message_id(*message.id())
        .delivery(Json(delivery.clone()))
        .plugin_id(*plugin_id)
        .success(result.is_ok())
        .created_at(Utc::now().naive_utc())
        .result(match result {
            Ok(response) => Some(MessageDispatchResult::new(response.reference.clone(), None)),
            Err(e) => None,
        })
        .build()
        .unwrap()
        .save(ctx.tx())
        .await
}

fn build_request<'r>() -> DispatchRequest<'r> {
    todo!()
}

async fn retrieve_message(pool: &PgPool, message_id: Id) -> Result<Message, CommandBusError> {
    Message::find(
        pool,
        MessageQueryBuilder::default()
            .id(Some(message_id))
            .build()
            .tap_err(|e| tracing::error!("Failed to build MessageQueryBuilder: {e}"))
            .map_err(anyhow::Error::from)?,
    )
    .await
    .tap_err(|e| tracing::error!("Failed to retrieve message from database: {e}"))?
    .tap_none(|| tracing::warn!("Message with id {} not found", message_id))
    .ok_or_else(|| Error::MessageNotFound(message_id).into())
}
