use super::dispatcher::MessageDispatcher;
use crate::{
    command_bus::{
        bus::CommandBusContext, commands::CommandType, handlers::CommandHandler, Result,
    },
    impl_command, into_event,
};
use async_trait::async_trait;
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{
    actor::Actor,
    id::Id,
    template::{TemplateData, TemplateRender},
};
use perroute_messaging::events::EventType;
use perroute_storage::{
    models::message::{Message, MessageQuery, Status},
    query::FetchableModel,
};
use serde::Serialize;
use sqlx::PgPool;
use std::sync::Arc;
use tap::{TapFallible, TapOptional};

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
pub enum DistributeMessageError {
    #[error("Message {0} not found")]
    MessageNotFound(Id),

    #[error("Invalid message {0} state")]
    InvalidMessageState(Id),
}

#[derive(Debug)]
pub struct DistributeMessageCommandHandler {
    template_render: Arc<dyn TemplateRender<TemplateData>>,
}

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
    ) -> Result<Self::Output> {
        let message = retrieve_message(ctx.pool(), cmd.message_id).await?;

        if Status::Pending != *message.status() {
            tracing::error!("To be distributed message must be in pending state");
            return Err(DistributeMessageError::InvalidMessageState(cmd.message_id).into());
        }

        let disp = MessageDispatcher::new(
            ctx.pool().clone(),
            ctx.plugins().clone(),
            message.clone(),
            self.template_render.clone(),
        )
        .await
        .unwrap();

        disp.execute().await;

        let message = message
            .set_status(Status::Distributed)
            .update(ctx.tx())
            .await
            .tap_err(|e| tracing::error!("Failed to update message: {e}"))?;

        Ok(message)
    }
}

async fn retrieve_message(pool: &PgPool, message_id: Id) -> Result<Message> {
    Message::find(pool, MessageQuery::with_id(message_id))
        .await
        .tap_err(|e| tracing::error!("Failed to retrieve message from database: {e}"))?
        .tap_none(|| tracing::warn!("Message with id {} not found", message_id))
        .ok_or_else(|| DistributeMessageError::MessageNotFound(message_id).into())
}
