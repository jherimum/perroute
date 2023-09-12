use std::sync::Arc;

use super::request::InnerDispatchrequest;
use crate::{
    command_bus::{
        bus::CommandBusContext, commands::CommandType, handlers::CommandHandler, Result,
    },
    impl_command, into_event,
};
use anyhow::Context;
use async_trait::async_trait;
use chrono::Utc;
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{
    actor::Actor,
    id::Id,
    template::{TemplateData, TemplateRender},
};
use perroute_connectors::{
    api::{DispatchError, DispatchResponse},
    types::plugin_id::ConnectorPluginId,
};
use perroute_connectors::{types::delivery::Delivery, Plugins};
use perroute_messaging::events::EventType;
use perroute_storage::{
    error::StorageError,
    models::{
        business_unit::BusinessUnit,
        message::{Message, MessageQuery, Status},
        message_dispatch::{MessageDispatch, MessageDispatchBuilder, MessageDispatchResult},
        message_type::MessageType,
        route::Route,
        schema::Schema,
        template::Template,
    },
    query::FetchableModel,
};
use serde::Serialize;
use sqlx::{types::Json, PgPool};
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
        let message = Arc::new(retrieve_message(ctx.pool(), cmd.message_id).await?);

        if Status::Pending != *message.status() {
            tracing::error!("To be distributed message must be in pending state");
            return Err(DistributeMessageError::InvalidMessageState(cmd.message_id).into());
        }

        let bu = Arc::new(message.business_unit(ctx.pool()).await?);
        let message_type = Arc::new(message.message_type(ctx.pool()).await?);
        let schema = Arc::new(message.schema(ctx.pool()).await.tap_err(|e| {
            tracing::error!("Failed to retrieve schema {}: {e}", message.schema_id())
        })?);

        let mut dispatches = vec![];
        for delivery in message.deliveries().iter() {
            let template = Arc::new(
                Template::find_active_template(
                    ctx.pool(),
                    *schema.id(),
                    delivery.dispatch_type(),
                    *message.created_at(),
                )
                .await?
                .unwrap(),
            );

            dispatches.push(tokio::spawn(dispatch_delivery(
                ctx.pool().clone(),
                ctx.plugins().clone(),
                message.clone(),
                schema.clone(),
                message_type.clone(),
                bu.clone(),
                template,
                delivery.clone(),
                self.template_render.clone(),
            )));
        }

        for dispatch in dispatches {
            let r = dispatch.await;
        }

        let message = message
            .as_ref()
            .clone()
            .set_status(Status::Distributed)
            .update(ctx.tx())
            .await
            .tap_err(|e| tracing::error!("Failed to update message: {e}"))?;

        Ok(message)
    }
}

async fn dispatch_delivery(
    pool: PgPool,
    plugins: Plugins,
    message: Arc<Message>,
    schema: Arc<Schema>,
    message_type: Arc<MessageType>,
    bu: Arc<BusinessUnit>,
    template: Arc<Template>,
    delivery: Delivery,
    template_render: Arc<dyn TemplateRender<TemplateData>>,
) -> Result<()> {
    for route in Route::dispatch_route_stack(&pool, schema.id(), &delivery.dispatch_type())
        .await
        .tap_err(|e| tracing::error!("Failed to retrieve routes: {e}"))?
    {
        let channel = route.channel(&pool).await?;
        let conn = route.connection(&pool).await?;
        let plugin = plugins.get(conn.plugin_id()).unwrap();
        let dispatcher = plugin.dispatcher(&delivery.dispatch_type()).unwrap();
        let disp_result = dispatcher
            .dispatch(Box::new(InnerDispatchrequest {
                id: Id::new(),
                delivery: delivery.clone(),
                message: message.clone(),
                schema: schema.clone(),
                message_type: message_type.clone(),
                business_unit: bu.clone(),
                connection: conn,
                channel: channel,
                route: route,
                template: template.clone(),
                template_render: template_render.clone(),
            }))
            .await;

        let message_dispatch =
            save_message_dispatch(&pool, &message, &delivery, &plugin.id(), disp_result).await?;

        if *message_dispatch.success() {
            break;
        }
    }

    Ok(())
}

async fn retrieve_message(pool: &PgPool, message_id: Id) -> Result<Message> {
    Message::find(pool, MessageQuery::with_id(message_id))
        .await
        .tap_err(|e| tracing::error!("Failed to retrieve message from database: {e}"))?
        .tap_none(|| tracing::warn!("Message with id {} not found", message_id))
        .ok_or_else(|| DistributeMessageError::MessageNotFound(message_id).into())
}

async fn save_message_dispatch(
    pool: &PgPool,
    message: &Message,
    delivery: &Delivery,
    plugin_id: &ConnectorPluginId,
    result: std::result::Result<DispatchResponse, DispatchError>,
) -> Result<MessageDispatch> {
    Ok(MessageDispatchBuilder::default()
        .id(Id::new())
        .message_id(*message.id())
        .delivery(Json(delivery.clone()))
        .plugin_id(*plugin_id)
        .success(result.is_ok())
        .created_at(Utc::now().naive_utc())
        .result(match result {
            Ok(response) => Some(MessageDispatchResult::new(response.reference.clone(), None)),
            Err(_) => None,
        })
        .build()
        .context("Failed to build message dispatch")?
        .save(pool)
        .await
        .tap_err(|e| tracing::error!("Failed to save message dispatch: {e}"))?)
}
