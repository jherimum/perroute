use crate::commands::channel::create_channel::CreateChannelError;
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
    ops::DerefMut,
    sync::Arc,
};
use tap::{TapFallible, TapOptional};
use tokio::sync::RwLock;

#[derive(Debug, thiserror::Error)]
pub enum CommandBusError {
    #[error("Command handler not found for command {0}")]
    HandlerNotFound(String),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),

    #[error(transparent)]
    CreateChannel(#[from] CreateChannelError),
}

#[derive(Clone, Debug)]
pub struct CommandBusContext<'tx> {
    pool: PgPool,
    tx: Arc<RwLock<Transaction<'tx, Postgres>>>,
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
        Ok(Self {
            pool,
            actor,
            tx: Arc::new(RwLock::new(tx)),
        })
    }

    pub fn actor(&self) -> &Actor {
        &self.actor
    }

    pub fn tx(&self) -> Arc<RwLock<Transaction<'tx, Postgres>>> {
        self.tx.clone()
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    async fn commit(self) -> Result<(), CommandBusError> {
        Arc::into_inner(self.tx)
            .unwrap()
            .into_inner()
            .commit()
            .await
            .with_context(|| "Failed to commit transaction")
            .map_err(CommandBusError::from)
    }
}

#[async_trait]
pub trait Command: Debug + Serialize + Clone + PartialEq + Eq + Send + Sync {
    fn name(&self) -> &str;
}

#[async_trait]
pub trait CommandHandler: Send + Sync + Debug {
    type Command: Command;
    async fn handle<'tx>(
        &self,
        ctx: &CommandBusContext<'tx>,
        cmd: Self::Command,
    ) -> Result<String, String>;
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
        ctx: &CommandBusContext<'tx>,
        cmd: &C,
    ) -> Result<(), CommandBusError> {
        CommandLog::new(cmd.name(), &cmd.clone(), ctx.actor())
            .save(ctx.tx().write().await.deref_mut())
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
            .tap_none(|| {
                tracing::error!("Handler not found for command: {}", cmd.name().to_string())
            })
            .ok_or_else(|| CommandBusError::HandlerNotFound(cmd.name().to_string()))
            .map_err(|_| "")?;

        let ctx = CommandBusContext::new(self.pool.clone(), actor)
            .await
            .map_err(|_| "")?;

        Self::log_command(&ctx, &cmd).await.map_err(|_| "")?;

        handler
            .handle(&ctx, cmd)
            .await
            .tap_err(|e| {
                tracing::error!("Failed to handle command: {e}"); //TODO: improve logging
            })
            .tap_ok(|event| {
                tracing::info!("Command handled successfully: {event:?}"); //TODO: improve logging
            })
            .map(|_| ())?; //TODO: publish event

        ctx.commit().await.map_err(|_| "")?;

        Ok(())
    }
}
