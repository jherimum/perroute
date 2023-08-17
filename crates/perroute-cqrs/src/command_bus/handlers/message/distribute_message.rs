use async_trait::async_trait;
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{actor::Actor, id::Id};
use perroute_messaging::events::EventType;
use perroute_storage::{
    models::{
        message::{Message, MessageQueryBuilder, Status},
        message_dispatch::{MessageDispatch, MessageDispatchBuilder, MessageDispatchStatus},
        route::{Route, RouteQueryBuilder},
    },
    query::FetchableModel,
};
use serde::Serialize;
use sqlx::{PgPool, Postgres, Transaction};
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
pub enum DistributeMessageCommandHandlerError {
    #[error("Message {0} not found")]
    MessageNotFound(Id),

    #[error("Invalid message state")]
    InvalidMessageState,
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
            return Err(
                DistributeMessageCommandHandlerError::MessageNotFound(cmd.message_id).into(),
            );
        }

        for route in fetch_routes(ctx.pool(), &message).await? {
            let dispatch = build_and_save_dispatch(ctx.tx(), &route, &message).await?;
            tracing::info!("Dispatch created: {:?}", dispatch);
        }

        let message = message
            .set_status(Status::Distributed)
            .update(ctx.tx())
            .await
            .tap_err(|e| tracing::error!("Failed to update message: {e}"))?;

        Ok(message)
    }
}

async fn build_and_save_dispatch<'tx>(
    tx: &mut Transaction<'tx, Postgres>,
    route: &Route,
    message: &Message,
) -> Result<MessageDispatch, CommandBusError> {
    MessageDispatchBuilder::default()
        .id(Id::new())
        .message_id(*message.id())
        .route_id(*route.id())
        .status(MessageDispatchStatus::Pending)
        .build()
        .unwrap()
        .save(tx)
        .await
        .tap_err(|e| tracing::error!("Failed to save message dispatch: {e}"))
        .map_err(CommandBusError::from)
}

async fn fetch_routes(pool: &PgPool, message: &Message) -> Result<Vec<Route>, CommandBusError> {
    Route::query(
        pool,
        RouteQueryBuilder::default()
            .schema_id(Some(*message.schema_id()))
            .build()
            .unwrap(),
    )
    .await
    .tap_err(|e| tracing::error!("Failed to retrieve routes from database: {e}"))
    .map_err(CommandBusError::from)
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
    .ok_or_else(|| DistributeMessageCommandHandlerError::MessageNotFound(message_id).into())
}
