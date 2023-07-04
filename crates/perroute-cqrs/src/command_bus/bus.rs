use super::{
    commands::Command,
    error::CommandBusError,
    handlers::{
        channel::{
            create_channel::CreateChannelCommandHandler,
            delete_channel::DeleteChannelCommandHandler,
            update_channel::UpdateChannelCommandHandler,
        },
        message_type::create_message_type::CreateMessageTypeCommandHandler,
        schema::create_schema::CreateSchemaCommandHandler,
        template::create_template::CreateTemplateCommandHandler,
        CommandHandler,
    },
};
use perroute_commons::types::actor::Actor;
use sqlx::{PgPool, Postgres, Transaction};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
    sync::Arc,
};
use tap::{TapFallible, TapOptional};

#[derive(Debug)]
pub struct CommandBusContext<'tx, 'a> {
    pool: PgPool,
    tx: Transaction<'tx, Postgres>,
    actor: &'a Actor,
}

impl<'tx, 'a> CommandBusContext<'tx, 'a> {
    pub async fn begin(
        pool: PgPool,
        actor: &'a Actor,
    ) -> Result<CommandBusContext<'tx, 'a>, CommandBusError> {
        let tx = pool
            .begin()
            .await
            .tap_err(|e| tracing::error!("Failed to begin transaction: {e}"))?;
        Ok(Self { pool, actor, tx })
    }

    pub const fn actor(&self) -> &Actor {
        self.actor
    }

    pub fn tx(&mut self) -> &mut Transaction<'tx, Postgres> {
        &mut self.tx
    }

    pub const fn pool(&self) -> &PgPool {
        &self.pool
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
    pub fn complete(pool: PgPool) -> Self {
        Self::builder()
            .with_pool(pool)
            .with_handler(CreateChannelCommandHandler)
            .with_handler(DeleteChannelCommandHandler)
            .with_handler(UpdateChannelCommandHandler)
            .with_handler(CreateMessageTypeCommandHandler)
            .with_handler(CreateSchemaCommandHandler)
            .with_handler(CreateSchemaCommandHandler)
            .with_handler(CreateSchemaCommandHandler)
            .with_handler(CreateSchemaCommandHandler)
            .with_handler(CreateTemplateCommandHandler)
            .build()
    }

    pub fn builder() -> CommandBusBuilder {
        Default::default()
    }

    fn get<C, H, O>(&self) -> Option<&H>
    where
        C: Command + 'static,
        H: CommandHandler<Command = C, Output = O> + 'static + Sync + Send,
        O: Debug,
    {
        let handler = self.handlers.get(&TypeId::of::<C>());
        handler.and_then(|h| h.downcast_ref::<H>())
    }

    pub async fn execute<C, H, O>(&self, actor: &Actor, cmd: &C) -> Result<O, CommandBusError>
    where
        C: Command + 'static,
        H: CommandHandler<Command = C, Output = O> + 'static + Sync + Send,
        O: Debug,
    {
        let handler = self
            .get::<C, H, O>()
            .tap_none(|| tracing::error!("Handler not found for command: {}", cmd.ty()))
            .ok_or_else(|| CommandBusError::HandlerNotFound(cmd.ty()))?;

        let mut ctx = CommandBusContext::begin(self.pool.clone(), actor)
            .await
            .tap_err(|e| tracing::error!("Failed to create command bus context: {e}"))?;

        let handler_result = handler
            .handle(&mut ctx, cmd.clone())
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

        ctx.commit()
            .await
            .tap_err(|e| tracing::error!("Failed to commit transaction: {e}"))?;

        handler_result
    }
}
