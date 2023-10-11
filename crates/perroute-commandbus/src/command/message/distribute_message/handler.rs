use anyhow::Context;
use async_trait::async_trait;
use chrono::Utc;
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{
    actor::Actor,
    command_type::CommandType,
    id::Id,
    template::{TemplateData, TemplateRender},
};
use perroute_connectors::{
    api::{DispatchError, DispatchResponse},
    types::{plugin_id::ConnectorPluginId, recipient::Recipient},
    Plugins,
};
use perroute_messaging::events::EventType;
use perroute_storage::{
    error::StorageError,
    models::{
        message::{Message, MessageQuery, Status},
        message_dispatch::{MessageDispatch, MessageDispatchBuilder, MessageDispatchResult},
        route::Route,
        template::Template,
    },
    query::FetchableModel,
};
use serde::Serialize;
use sqlx::{types::Json, PgPool};
use std::sync::Arc;
use tap::{TapFallible, TapOptional};

use crate::{bus::Ctx, command::Command, error::CommandBusError};

use super::request::InnerDispatchRequest;

#[derive(Debug, derive_builder::Builder)]
pub struct DistributeMessageCommand {
    message_id: Id,
}

#[derive(Debug, thiserror::Error)]
pub enum DistributeMessageError {
    #[error("Message {0} not found")]
    MessageNotFound(Id),

    #[error("Invalid message {0} state")]
    InvalidMessageState(Id),

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),

    #[error(transparent)]
    StorageError(#[from] StorageError),
}

#[async_trait]
impl Command for DistributeMessageCommand {
    type Output = Message;

    #[tracing::instrument(name = "distribute_message_handler", skip(self, ctx))]
    async fn handle<'tx>(&self, ctx: &mut Ctx<'tx>) -> Result<Self::Output, CommandBusError> {
        // let message = retrieve_message(ctx.pool(), cmd.message_id).await?;

        // if Status::Pending != *message.status() {
        //     tracing::error!("To be distributed message must be in pending state");
        //     return Err(DistributeMessageError::InvalidMessageState(cmd.message_id).into());
        // }

        // let bu = message.business_unit(ctx.pool()).await?;
        // let message_type = message.message_type(ctx.pool()).await?;

        // let mut dispatches = vec![];

        // for delivery in message.deliveries().iter() {
        //     let template = active_template(ctx.pool(), &schema, &message, delivery).await?;

        //     if template.is_none() {
        //         tracing::warn!(
        //             "No template found for message {} and delivery {}",
        //             message.id(),
        //             delivery
        //         );
        //         continue;
        //     }

        //     for route in routes(ctx.pool(), &schema, delivery).await? {
        //         let channel = route.channel(ctx.pool()).await?;
        //         let conn = route.connection(ctx.pool()).await?;
        //         let plugin = ctx.plugins().get(conn.plugin_id()).unwrap();
        //         let dispatcher = plugin.dispatcher(&delivery.dispatch_type()).unwrap();

        //         let result = dispatcher
        //             .dispatch(Box::new(InnerDispatchRequest {
        //                 id: Id::new(),
        //                 delivery: delivery.clone(),
        //                 message: message.clone(),
        //                 schema: schema.clone(),
        //                 message_type: message_type.clone(),
        //                 business_unit: bu.clone(),
        //                 connection: conn,
        //                 channel,
        //                 route: route.clone(),
        //                 template: template.as_ref().unwrap().clone(),
        //                 template_render: self.template_render.clone(),
        //             }))
        //             .await;

        //         dispatches.push(
        //             save_message_dispatch(ctx.pool(), &message, delivery, plugin.id(), &result)
        //                 .await?,
        //         );

        //         match result {
        //             Ok(_) => {
        //                 tracing::info!(
        //                     "Message {} successfully dispatched to {}",
        //                     message.id(),
        //                     delivery
        //                 );
        //                 break;
        //             }
        //             Err(e) => tracing::error!(
        //                 "Failed to dispatch message {} to {}: {}",
        //                 message.id(),
        //                 delivery,
        //                 e
        //             ),
        //         }
        //     }
        // }

        // let message = message
        //     .set_status(Status::Distributed)
        //     .update(ctx.pool())
        //     .await
        //     .tap_err(|e| tracing::error!("Failed to update message: {e}"))?;

        // Ok(message)
        todo!()
    }

    fn command_type(&self) -> CommandType {
        CommandType::DistributeMessage
    }

    fn supports(&self, _actor: &Actor) -> bool {
        true
    }
}

// async fn active_template(
//     pool: &PgPool,
//     schema: &Schema,
//     message: &Message,
//     delivery: &Delivery,
// ) -> Result<Option<Template>> {
//     Ok(Template::find_active_template(
//         pool,
//         schema.id(),
//         &delivery.dispatch_type(),
//         message.created_at(),
//     )
//     .await?)
// }

// async fn routes(pool: &PgPool, schema: &Schema, delivery: &Delivery) -> Result<Vec<Route>> {
//     Ok(Route::dispatch_route_stack(pool, schema.id(), &delivery.dispatch_type()).await?)
// }

// async fn save_message_dispatch(
//     pool: &PgPool,
//     message: &Message,
//     delivery: &Delivery,
//     plugin_id: ConnectorPluginId,
//     result: &std::result::Result<DispatchResponse, DispatchError>,
// ) -> std::result::Result<MessageDispatch, DistributeMessageError> {
//     Ok(MessageDispatchBuilder::default()
//         .id(Id::new())
//         .message_id(*message.id())
//         .delivery(Json(delivery.clone()))
//         .plugin_id(plugin_id)
//         .success(result.is_ok())
//         .created_at(Utc::now().naive_utc())
//         .result(match result {
//             Ok(response) => Some(MessageDispatchResult::new(response.reference.clone(), None)),
//             Err(_) => None,
//         })
//         .build()
//         .context("Failed to build message dispatch")?
//         .save(pool)
//         .await
//         .tap_err(|e| tracing::error!("Failed to save message dispatch: {e}"))?)
// }

// async fn retrieve_message(pool: &PgPool, message_id: Id) -> Result<Message> {
//     Message::find(pool, MessageQuery::with_id(message_id))
//         .await
//         .tap_err(|e| tracing::error!("Failed to retrieve message from database: {e}"))?
//         .tap_none(|| tracing::warn!("Message with id {} not found", message_id))
//         .ok_or_else(|| DistributeMessageError::MessageNotFound(message_id).into())
// }
