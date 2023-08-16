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
        message::create_message::CreateMessageCommandHandler,
        message_type::{
            create_message_type::CreateMessageTypeCommandHandler,
            delete_message_type::DeleteMessageTypeCommandHandler,
            update_message_type::UpdateMessageTypeCommandHandler,
        },
        schema::{
            create_schema::CreateSchemaCommandHandler, delete_schema::DeleteSchemaCommandHandler,
            update_schema::UpdateSchemaCommandHandler,
        },
        template::{
            activate_template::ActivateTemplateCommandHandler,
            create_template::CreateTemplateCommandHandler,
            delete_template::DeleteTemplateCommandHandler,
            update_template::UpdateTemplateCommandHandler,
        },
        CommandHandler,
    },
};
use perroute_commons::types::actor::Actor;
use perroute_connectors::Plugins;
use perroute_storage::models::db_event::DbEvent;
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
    pool: PgPool,
    tx: Transaction<'tx, Postgres>,
}

impl<'tx> CommandBusContext<'tx> {
    pub async fn begin(pool: PgPool) -> Result<CommandBusContext<'tx>, CommandBusError> {
        let tx = pool
            .begin()
            .await
            .tap_err(|e| tracing::error!("Failed to begin transaction: {e}"))?;
        Ok(Self { pool, tx })
    }

    pub fn tx(&mut self) -> &mut Transaction<'tx, Postgres> {
        &mut self.tx
    }

    pub const fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn conn(&mut self) -> &mut PgConnection {
        self.tx.acquire().await.unwrap()
    }

    async fn commit(self) -> Result<(), CommandBusError> {
        self.tx
            .commit()
            .await
            .tap_err(|e| tracing::error!("Failed to commit transaction: {e}"))
            .map_err(Into::into)
    }
}

#[derive(Default)]
pub struct CommandBusBuilder {
    pool: Option<PgPool>,
    handlers: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl CommandBusBuilder {
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
            pool: self.pool.expect("Pool is required"),
            handlers: Arc::new(self.handlers),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CommandBus {
    pool: PgPool,
    handlers: Arc<HashMap<TypeId, Box<dyn Any + Send + Sync>>>,
}

impl CommandBus {
    pub fn complete(pool: PgPool, plugins: Plugins) -> Self {
        Self::builder()
            .with_pool(pool)
            .with_handler(CreateBusinessUnitCommandHandler)
            .with_handler(DeleteBusinessUnitCommandHandler)
            .with_handler(UpdateBusinessUnitCommandHandler)
            .with_handler(CreateMessageTypeCommandHandler)
            .with_handler(UpdateMessageTypeCommandHandler)
            .with_handler(DeleteMessageTypeCommandHandler)
            .with_handler(CreateSchemaCommandHandler)
            .with_handler(CreateSchemaCommandHandler)
            .with_handler(DeleteSchemaCommandHandler)
            .with_handler(UpdateSchemaCommandHandler)
            .with_handler(CreateTemplateCommandHandler)
            .with_handler(UpdateTemplateCommandHandler)
            .with_handler(DeleteTemplateCommandHandler)
            .with_handler(ActivateTemplateCommandHandler)
            .with_handler(CreateMessageCommandHandler)
            .with_handler(CreateConnectionCommandHandler {
                plugins: plugins.clone(),
            })
            .with_handler(UpdateConnectionCommandHandler {
                plugins: plugins.clone(),
            })
            .with_handler(DeleteConnectionCommandHandler {
                plugins: plugins.clone(),
            })
            .with_handler(CreateChannelCommandHandler {
                plugins: plugins.clone(),
            })
            .with_handler(UpdateChannelCommandHandler {
                plugins: plugins.clone(),
            })
            .with_handler(DeleteChannelCommandHandler {
                plugins: plugins.clone(),
            })
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

    pub async fn execute<C, H, O>(&self, actor: &Actor, cmd: &C) -> Result<O, CommandBusError>
    where
        C: Command + 'static,
        H: CommandHandler<Command = C, Output = O> + 'static + Sync + Send,
        O: Debug + Send + Sync,
    {
        let handler = self
            .get::<C, H, O>()
            .tap_none(|| tracing::error!("Handler not found for command: {}", cmd.ty()))
            .ok_or_else(|| CommandBusError::HandlerNotFound(cmd.ty()))?;

        let mut ctx = CommandBusContext::begin(self.pool.clone())
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
