use crate::command_bus::handlers::message::distribute_message::request::InnerDispatchrequest;
use anyhow::Context;
use chrono::Utc;
use perroute_commons::types::{
    id::Id,
    template::{TemplateData, TemplateRender},
};
use perroute_connectors::{
    api::{DispatchError, DispatchResponse},
    types::{delivery::Delivery, plugin_id::ConnectorPluginId},
    Plugins,
};
use perroute_storage::{
    error::StorageError,
    models::{
        business_unit::BusinessUnit,
        message::Message,
        message_dispatch::{self, MessageDispatch, MessageDispatchBuilder, MessageDispatchResult},
        message_type::MessageType,
        route::Route,
        schema::Schema,
        template::Template,
    },
};
use sqlx::{types::Json, PgPool};
use std::sync::Arc;
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum MessageDispatcherError {
    #[error(transparent)]
    StorageError(#[from] StorageError),

    #[error("")]
    AnyTemplateElegible,

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[derive(Clone)]
pub struct MessageDispatcher {
    pool: PgPool,
    plugins: Plugins,
    message: Message,
    template_render: Arc<dyn TemplateRender<TemplateData>>,
    schema: Schema,
    message_type: MessageType,
    bu: BusinessUnit,
}

impl MessageDispatcher {
    pub async fn new(
        pool: PgPool,
        plugins: Plugins,
        message: Message,
        template_render: Arc<dyn TemplateRender<TemplateData>>,
    ) -> Result<Self, MessageDispatcherError> {
        let bu = message.business_unit(&pool).await?;
        let message_type = message.message_type(&pool).await?;
        let schema = message.schema(&pool).await?;

        Ok(Self {
            pool,
            plugins,
            message,
            template_render,
            schema,
            message_type,
            bu,
        })
    }

    pub async fn execute(self) {
        for delivery in self.message.deliveries().iter() {
            match DeliveryDispatcher::new(self.clone(), delivery.clone()).await {
                Ok(d) => match tokio::spawn(d.execute()).await {
                    Ok(r) => match r {
                        Ok(_) => tracing::info!("Dispatch for delivery {} completed", delivery),
                        Err(e) => tracing::error!("Failed: {e}"),
                    },
                    Err(e) => tracing::error!(
                        "Failed to join dispatch task for delivery {}: {e}",
                        delivery
                    ),
                },
                Err(e) => {
                    tracing::error!("Failed to create dispath for delivery {}: {e}", delivery)
                }
            }
        }
    }
}

pub struct DeliveryDispatcher {
    message_dispatcher: MessageDispatcher,
    delivery: Delivery,
    template: Template,
}

impl DeliveryDispatcher {
    fn pool(&self) -> &PgPool {
        &self.message_dispatcher.pool
    }

    fn plugins(&self) -> &Plugins {
        &self.message_dispatcher.plugins
    }

    async fn new(
        dispatcher: MessageDispatcher,
        delivery: Delivery,
    ) -> Result<Self, MessageDispatcherError> {
        let template = Template::find_active_template(
            &dispatcher.pool,
            dispatcher.schema.id(),
            &delivery.dispatch_type(),
            dispatcher.message.created_at(),
        )
        .await
        .tap_err(|e| tracing::error!("Failed to retrieve template: {e}"))?
        .ok_or(MessageDispatcherError::AnyTemplateElegible)?;

        Ok(Self {
            message_dispatcher: dispatcher,
            delivery,
            template,
        })
    }

    async fn routes(&self) -> Result<Vec<Route>, MessageDispatcherError> {
        Ok(Route::dispatch_route_stack(
            &self.message_dispatcher.pool,
            self.message_dispatcher.schema.id(),
            &self.delivery.dispatch_type(),
        )
        .await
        .tap_err(|e| tracing::error!("Failed to retrieve routes: {e}"))?)
    }

    async fn execute(self) -> Result<Vec<MessageDispatch>, MessageDispatcherError> {
        let message_dispatches = vec![];

        for route in self.routes().await? {
            self.dispatch(route, &self.template).await;
        }

        Ok(message_dispatches)
    }

    async fn dispatch(
        &self,
        route: Route,
        template: &Template,
    ) -> Result<MessageDispatch, MessageDispatcherError> {
        let channel = route.channel(self.pool()).await?;
        let conn = route.connection(self.pool()).await?;
        let plugin = self.plugins().get(conn.plugin_id()).unwrap();
        let dispatcher = plugin.dispatcher(&self.delivery.dispatch_type()).unwrap();

        let disp_result = dispatcher
            .dispatch(Box::new(InnerDispatchrequest {
                id: Id::new(),
                delivery: self.delivery.clone(),
                message: self.message_dispatcher.message.clone(),
                schema: self.message_dispatcher.schema.clone(),
                message_type: self.message_dispatcher.message_type.clone(),
                business_unit: self.message_dispatcher.bu.clone(),
                connection: conn,
                channel: channel,
                route: route,
                template: template.clone(),
                template_render: self.message_dispatcher.template_render.clone(),
            }))
            .await;

        self.save_message_dispatch(plugin.id(), disp_result).await
    }

    async fn save_message_dispatch(
        &self,
        plugin_id: ConnectorPluginId,
        result: std::result::Result<DispatchResponse, DispatchError>,
    ) -> std::result::Result<MessageDispatch, MessageDispatcherError> {
        Ok(MessageDispatchBuilder::default()
            .id(Id::new())
            .message_id(*self.message_dispatcher.message.id())
            .delivery(Json(self.delivery.clone()))
            .plugin_id(plugin_id)
            .success(result.is_ok())
            .created_at(Utc::now().naive_utc())
            .result(match result {
                Ok(response) => Some(MessageDispatchResult::new(response.reference.clone(), None)),
                Err(_) => None,
            })
            .build()
            .context("Failed to build message dispatch")?
            .save(self.pool())
            .await
            .tap_err(|e| tracing::error!("Failed to save message dispatch: {e}"))?)
    }
}
