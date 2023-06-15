use crate::commands::{channel::create_channel::CreateChannelError, CommandType};
use anyhow::Context;
use async_trait::async_trait;
use perroute_commons::types::actor::Actor;
use perroute_storage::models::command_log::CommandLog;
use serde::Serialize;
use sqlx::{PgPool, Postgres, Transaction};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
    sync::Arc,
};
use tap::{TapFallible, TapOptional};

#[derive(Debug, thiserror::Error)]
pub enum CommandBusError {
    #[error("Command handler not found for command {0}")]
    HandlerNotFound(CommandType),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),

    #[error(transparent)]
    CreateChannel(#[from] CreateChannelError),
}

#[derive(Debug)]
pub struct CommandBusContext<'tx> {
    pool: PgPool,
    tx: Transaction<'tx, Postgres>,
    actor: Actor,
}

impl<'tx> CommandBusContext<'tx> {
    pub async fn new(
        pool: PgPool,
        actor: Actor,
    ) -> Result<CommandBusContext<'tx>, CommandBusError> {
        let tx = pool
            .begin()
            .await
            .with_context(|| "Failed to begin transaction")?;
        Ok(Self { pool, actor, tx })
    }

    pub fn actor(&self) -> &Actor {
        &self.actor
    }

    pub fn tx(&mut self) -> &mut Transaction<'tx, Postgres> {
        &mut self.tx
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    async fn commit(self) -> Result<(), CommandBusError> {
        self.tx
            .commit()
            .await
            .with_context(|| "")
            .map_err(CommandBusError::from)
    }
}

#[async_trait]
pub trait Command:
    Debug + Serialize + Clone + PartialEq + Eq + Send + Sync + From<CommandLog<Self>>
{
    fn ty(&self) -> CommandType;

    fn into_log(&self, error: Option<Box<dyn std::error::Error>>) -> CommandLog<Self> {
        CommandLog::new(self.ty(), self, &Actor::system(), error)
    }
}

#[async_trait]
pub trait CommandHandler: Send + Sync + Debug {
    type Command: Command;
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        cmd: &Self::Command,
    ) -> Result<(), CommandBusError>;
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

pub struct CommandBus {
    pool: PgPool,
    handlers: Arc<HashMap<TypeId, Box<dyn Any + Send + Sync>>>,
}

impl CommandBus {
    pub fn builder() -> CommandBusBuilder {
        Default::default()
    }

    fn get<C, H>(&self) -> Option<&H>
    where
        C: Command + 'static,
        H: CommandHandler<Command = C> + 'static + Sync + Send,
    {
        let handler = self.handlers.get(&TypeId::of::<C>());
        handler.and_then(|h| h.downcast_ref::<H>())
    }

    #[allow(clippy::await_holding_lock)]
    async fn log_command<'tx, C: Command>(
        &self,
        cmd: &C,
        actor: &Actor,
        error: Option<Box<dyn std::error::Error>>,
    ) -> Result<(), CommandBusError> {
        cmd.into_log(error)
            .save(&self.pool)
            .await
            .tap_err(|e| tracing::error!("Failed to save command log: {e}"))
            .tap_ok(|_| {
                tracing::info!("Command log saved successfully"); //TODO: improve logging
            })
            .with_context(|| "Failed to save command log")
            .map_err(CommandBusError::from)
            .map(|_| ())
    }

    pub async fn execute<C, H>(&self, actor: Actor, cmd: C) -> Result<(), String>
    where
        C: Command + 'static,
        H: CommandHandler<Command = C> + 'static + Sync + Send,
    {
        let handler = self
            .get::<C, H>()
            .tap_none(|| tracing::error!("Handler not found for command: {}", cmd.ty()))
            .ok_or_else(|| CommandBusError::HandlerNotFound(cmd.ty()))
            .map_err(|_| "")?;

        let mut ctx = CommandBusContext::new(self.pool.clone(), actor.clone())
            .await
            .map_err(|_| "")?;

        let handler_result = handler
            .handle(&mut ctx, &cmd)
            .await
            .tap_err(|e| {
                tracing::error!("Failed to handle command: {e}"); //TODO: improve logging
            })
            .tap_ok(|event| {
                tracing::info!("Command handled successfully: {event:?}"); //TODO: improve logging
            })
            .map(|_| ()); //TODO: publish event

        ctx.commit().await.map_err(|_| "")?;

        self.log_command(&cmd, &actor, None).await.map_err(|_| "")?;

        Ok(())
    }
}
