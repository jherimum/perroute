use super::{
    commands::Command,
    error::CommandBusError,
    handlers::{
        business_unit::{
            create_business_unit::CreateBusinessUnitCommandHandler,
            delete_business_unit::DeleteBusinessUnitCommandHandler,
            update_business_unit::UpdateBusinessUnitCommandHandler,
        },
        channel::{
            create_channel::CreateChannelCommandHandler,
            delete_channel::DeleteChannelCommandHandler,
            update_channel::UpdateChannelCommandHandler,
        },
        connection::{
            create_connection::CreateConnectionCommandHandler,
            delete_connection::DeleteConnectionCommandHandler,
            update_connection::UpdateConnectionCommandHandler,
        },
        message::{
            create_message::CreateMessageCommandHandler,
            distribute_message::handler::DistributeMessageCommandHandler,
        },
        message_type::{
            create_message_type::CreateMessageTypeCommandHandler,
            delete_message_type::DeleteMessageTypeCommandHandler,
            update_message_type::UpdateMessageTypeCommandHandler,
        },
        route::{
            create_route::CreateRouteCommandHandler, delete_route::DeleteRouteCommandHandler,
            update_route::UpdateRouteCommandHandler,
        },
        schema::{
            create_schema::CreateSchemaCommandHandler, delete_schema::DeleteSchemaCommandHandler,
            update_schema::UpdateSchemaCommandHandler,
        },
        template::{
            create_template::CreateTemplateCommandHandler,
            delete_template::DeleteTemplateCommandHandler,
            update_template::UpdateTemplateCommandHandler,
        },
        CommandHandler,
    },
    Result,
};
use perroute_commons::types::{
    actor::Actor,
    template::{TemplateData, TemplateRender},
};
use perroute_connectors::Plugins;
use perroute_storage::{error::StorageError, models::db_event::DbEvent};
use sqlx::{Acquire, PgConnection, PgPool, Postgres, Transaction};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
    sync::Arc,
};
use tap::{TapFallible, TapOptional};

#[derive(Debug)]
pub struct CommandBusContext<'tx> {
    plugins: Plugins,
    pool: PgPool,
    tx: Transaction<'tx, Postgres>,
}

impl<'tx> CommandBusContext<'tx> {
    pub async fn begin(pool: PgPool, plugins: Plugins) -> Result<CommandBusContext<'tx>> {
        let tx = pool
            .begin()
            .await
            .tap_err(|e| tracing::error!("Failed to begin transaction: {e}"))
            .map_err(StorageError::Tx)?;
        Ok(Self { plugins, pool, tx })
    }

    pub fn tx(&mut self) -> &mut Transaction<'tx, Postgres> {
        &mut self.tx
    }

    pub const fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub fn plugins(&self) -> &Plugins {
        &self.plugins
    }

    pub async fn conn(&mut self) -> &mut PgConnection {
        self.tx.acquire().await.unwrap()
    }

    async fn commit(self) -> Result<()> {
        Ok(self
            .tx
            .commit()
            .await
            .tap_err(|e| tracing::error!("Failed to commit transaction: {e}"))
            .map_err(StorageError::Tx)?)
    }
}

#[derive(Default)]
pub struct CommandBusBuilder {
    plugins: Option<Plugins>,
    pool: Option<PgPool>,
    handlers: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl CommandBusBuilder {
    pub fn with_plugins(mut self, plugins: Plugins) -> Self {
        self.plugins = Some(plugins);
        self
    }

    pub fn with_pool(mut self, pool: PgPool) -> Self {
        self.pool = Some(pool);
        self
    }

    pub fn with_handler<C, H>(mut self, handler: H) -> Self
    where
        C: Command + 'static,
        H: CommandHandler<Command = C> + 'static + Sync + Send,
    {
        self.handlers.insert(TypeId::of::<C>(), Box::new(handler));
        self
    }

    pub fn build(self) -> CommandBus {
        CommandBus {
            plugins: self.plugins.expect("Plugins are required"),
            pool: self.pool.expect("Pool is required"),
            handlers: Arc::new(self.handlers),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CommandBus {
    plugins: Plugins,
    pool: PgPool,
    handlers: Arc<HashMap<TypeId, Box<dyn Any + Send + Sync>>>,
}

impl CommandBus {
    pub fn complete(
        pool: PgPool,
        plugins: Plugins,
        template_render: Arc<dyn TemplateRender<TemplateData>>,
    ) -> Self {
        Self::builder()
            .with_plugins(plugins.clone())
            .with_pool(pool.clone())
            //business unit
            .with_handler(CreateBusinessUnitCommandHandler::new(pool.clone()))
            .with_handler(DeleteBusinessUnitCommandHandler::new(pool.clone()))
            .with_handler(UpdateBusinessUnitCommandHandler::new(pool.clone()))
            //message type
            .with_handler(CreateMessageTypeCommandHandler::new(pool.clone()))
            .with_handler(UpdateMessageTypeCommandHandler::new(pool.clone()))
            .with_handler(DeleteMessageTypeCommandHandler::new(pool.clone()))
            //schema
            .with_handler(CreateSchemaCommandHandler::new(pool.clone()))
            .with_handler(DeleteSchemaCommandHandler::new(pool.clone()))
            .with_handler(UpdateSchemaCommandHandler::new(pool.clone()))
            //template
            .with_handler(CreateTemplateCommandHandler::new(pool.clone()))
            .with_handler(UpdateTemplateCommandHandler::new(pool.clone()))
            .with_handler(DeleteTemplateCommandHandler::new(pool.clone()))
            //message
            .with_handler(CreateMessageCommandHandler::new(pool.clone()))
            .with_handler(DistributeMessageCommandHandler::new(
                pool.clone(),
                plugins.clone(),
                template_render.clone(),
            ))
            //opnnection
            .with_handler(CreateConnectionCommandHandler::new(
                pool.clone(),
                plugins.clone(),
            ))
            .with_handler(UpdateConnectionCommandHandler::new(
                pool.clone(),
                plugins.clone(),
            ))
            .with_handler(DeleteConnectionCommandHandler::new(pool.clone()))
            //channel
            .with_handler(CreateChannelCommandHandler::new(
                pool.clone(),
                plugins.clone(),
            ))
            .with_handler(UpdateChannelCommandHandler::new(
                pool.clone(),
                plugins.clone(),
            ))
            .with_handler(DeleteChannelCommandHandler::new(pool.clone()))
            //route
            .with_handler(CreateRouteCommandHandler::new(pool.clone()))
            .with_handler(UpdateRouteCommandHandler::new(pool.clone()))
            .with_handler(DeleteRouteCommandHandler::new(pool.clone()))
            .build()
    }

    pub fn builder() -> CommandBusBuilder {
        CommandBusBuilder::default()
    }

    fn get<C, H, O>(&self) -> Option<&H>
    where
        C: Command + 'static,
        H: CommandHandler<Command = C, Output = O> + 'static + Sync + Send,
        O: Debug + Send + Sync,
    {
        let handler = self.handlers.get(&TypeId::of::<C>());
        handler.and_then(|h| h.downcast_ref::<H>())
    }

    pub async fn execute<C, H, O>(&self, actor: &Actor, cmd: &C) -> Result<O>
    where
        C: Command + 'static,
        H: CommandHandler<Command = C, Output = O> + 'static + Sync + Send,
        O: Debug + Send + Sync,
    {
        let handler = self
            .get::<C, H, O>()
            .tap_none(|| tracing::error!("Handler not found for command: {}", cmd.ty()))
            .ok_or_else(|| CommandBusError::HandlerNotFound(cmd.ty()))?;

        let mut ctx = CommandBusContext::begin(self.pool.clone(), self.plugins.clone())
            .await
            .tap_err(|e| tracing::error!("Failed to create command bus context: {e}"))?;

        let handler_result = handler
            .handle(&mut ctx, actor, cmd.clone())
            .await
            .tap_err(|e| {
                tracing::error!("Failed to handle command: {e}"); //TODO: improve logging
            })
            .tap_ok(|event| {
                tracing::info!("Command handled successfully: {event:?}"); //TODO: improve logging
            });

        cmd.to_log(actor, handler_result.as_ref().err())
            .save(ctx.tx())
            .await
            .tap_err(|e| tracing::error!("Failed to save command log: {e}"))?;

        if handler_result.is_ok() {
            if let Some(event) = cmd.into_event() {
                let db_event: DbEvent = (&event).into();
                db_event.save(ctx.tx()).await?;
            }
        }

        ctx.commit()
            .await
            .tap_err(|e| tracing::error!("Failed to commit transaction: {e}"))?;

        handler_result
    }
}
