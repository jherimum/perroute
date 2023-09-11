use async_trait::async_trait;
use chrono::Utc;
use derive_builder::Builder;
use derive_getters::Getters;
use futures::Future;
use perroute_commons::{
    new_id,
    types::{
        actor::Actor,
        id::Id,
        template::{TemplateData, TemplateError},
        vars::Vars,
    },
};
use perroute_connectors::{
    api::{DispatchError, DispatchRequest, DispatchResponse},
    template::DispatchTemplate,
    types::plugin_id::ConnectorPluginId,
};
use perroute_connectors::{types::delivery::Delivery, Plugins};
use perroute_messaging::events::EventType;
use perroute_storage::{
    models::{
        channel::Channel,
        connection::Connection,
        message::{Message, MessageQueryBuilder, Status},
        message_dispatch::{MessageDispatch, MessageDispatchBuilder, MessageDispatchResult},
        route::Route,
        schema::Schema,
        template::Template,
    },
    query::FetchableModel,
};
use serde::Serialize;
use sqlx::{types::Json, PgPool};
use std::{
    collections::HashMap,
    pin::Pin,
    task::{Context, Poll},
};
use tap::{TapFallible, TapOptional};
use tokio::task::JoinHandle;

use crate::{
    command_bus::{
        bus::CommandBusContext,
        commands::CommandType,
        error::CommandBusError,
        handlers::{channel, CommandHandler},
        Result,
    },
    impl_command, into_event,
};

use super::template::InnerDispatchTemplate;

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
    ) -> Result<Self::Output> {
        let message = retrieve_message(ctx.pool(), cmd.message_id).await?;

        if Status::Pending != *message.status() {
            tracing::error!("To be distributed message must be in pending state");
            return Err(DistributeMessageError::InvalidMessageState(cmd.message_id).into());
        }

        let schema = message.schema(ctx.pool()).await.tap_err(|e| {
            tracing::error!("Failed to retrieve schema {}: {e}", message.schema_id())
        })?;

        //let mut dispatches = vec![];
        for delivery in message.deliveries().iter() {
            let template = Template::find_active_template(
                ctx.pool(),
                *schema.id(),
                delivery.dispatch_type(),
                *message.created_at(),
            )
            .await?
            .unwrap();

            // dispatches.push(tokio::spawn(dispatch_delivery(
            //     ctx.pool().clone(),
            //     ctx.plugins().clone(),
            //     message.clone(),
            //     schema.clone(),
            //     template,
            //     delivery.clone(),
            // )));
        }

        // for dispatch in dispatches {
        //     let r = dispatch.await;
        // }

        let message = message
            .set_status(Status::Distributed)
            .update(ctx.tx())
            .await
            .tap_err(|e| tracing::error!("Failed to update message: {e}"))?;

        Ok(message)
    }
}

fn build_request<'r>(
    message: &'r Message,
    message_dispatch: &'r MessageDispatch,
    connection: &'r Connection,
    route: &'r Route,
    channel: &'r Channel,
    template: &'r dyn DispatchTemplate,
    vars: &'r Vars,
) -> DispatchRequest<'r> {
    DispatchRequest {
        id: *message_dispatch.id(),
        connection_properties: connection.properties(),
        dispatch_properties: channel.properties(),
        template: template,
        payload: message.payload(),
        vars: vars,
        delivery: message_dispatch.delivery().as_ref().clone(),
    }
}

async fn retrieve_message(pool: &PgPool, message_id: Id) -> Result<Message> {
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
    .ok_or_else(|| DistributeMessageError::MessageNotFound(message_id).into())
}

async fn dispatch_delivery<'tx>(
    pool: PgPool,
    plugins: Plugins,
    message: Message,
    schema: Schema,
    template: Template,
    delivery: Delivery,
) -> Result<()> {
    for route in Route::dispatch_route_stack(&pool, schema.id(), &delivery.dispatch_type()).await? {
        let channel = route.channel(&pool).await?;
        let conn = route.connection(&pool).await?;
        let plugin = plugins.get(conn.plugin_id()).unwrap();
        let dispatcher = plugin.dispatcher(&delivery.dispatch_type()).unwrap();
        let message_dispatch = build_message_dispatch(&message, &delivery, &plugin.id())
            .save(&pool)
            .await
            .unwrap();

        let temp = InnerDispatchTemplate(&template);
        let vars = &Vars::default();

        let request = build_request(
            &message,
            &message_dispatch,
            &conn,
            &route,
            &channel,
            &temp,
            vars,
        );
        let disp_result = dispatcher.dispatch(&request).await;

        message_dispatch
            .set_success(true)
            .set_result(Option::default())
            .update(&pool)
            .await;

        if disp_result.is_ok() {
            break;
        }
    }

    Ok(())
}

fn build_message_dispatch(
    message: &Message,
    delivery: &Delivery,
    plugin_id: &ConnectorPluginId,
    //result: &std::result::Result<DispatchResponse, DispatchError>,
) -> MessageDispatch {
    MessageDispatchBuilder::default()
        .id(Id::new())
        .message_id(*message.id())
        .delivery(Json(delivery.clone()))
        .plugin_id(*plugin_id)
        //.success(result.is_ok())
        .created_at(Utc::now().naive_utc())
        // .result(match result {
        //     Ok(response) => Some(MessageDispatchResult::new(response.reference.clone(), None)),
        //     Err(_) => None,
        // })
        .build()
        .unwrap()
}
