use crate::{command::Command, error::CommandBusError};
use perroute_commons::types::actor::Actor;
use sqlx::PgPool;
use std::marker::PhantomData;
use tap::TapFallible;

#[derive(Clone)]
pub struct CommandBus {
    pool: PgPool,
}

impl CommandBus {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl CommandBus {
    pub async fn execute<C, O>(&self, actor: Actor, command: C) -> Result<O, CommandBusError>
    where
        C: Command<Output = O>,
    {
        InnerExecutor::new(self.pool.clone(), actor, command)
            .execute()
            .await
    }
}

struct InnerExecutor<C, O> {
    pool: PgPool,
    actor: Actor,
    command: C,
    output: PhantomData<O>,
}

impl<C, O> InnerExecutor<C, O>
where
    C: Command<Output = O>,
{
    pub fn new(pool: PgPool, actor: Actor, command: C) -> Self {
        Self {
            pool,
            actor,
            command,
            output: PhantomData,
        }
    }

    pub async fn execute(self) -> Result<O, CommandBusError> {
        if !self.command.supports(&self.actor) {
            tracing::error!(
                "Actor [{:?}] is not allowed to execute command [{:?}]",
                self.actor,
                self.command.command_type()
            );

            return Err(CommandBusError::ActorNotSupported);
        };

        let mut ctx = Ctx::new(&self.pool, &self.actor).await?;
        self.command
            .handle(&mut ctx)
            .await
            .tap_ok(|_| tracing::info!("Command [{:?}] handled successfully", self.command))
            .tap_err(|e| tracing::error!("Failed to handle command [{:?}]: {e}", self.command))
    }
}

pub struct Ctx<'ctx> {
    pool: &'ctx PgPool,
    actor: &'ctx Actor,
}

impl<'ctx> Ctx<'ctx> {
    pub async fn new(pool: &'ctx PgPool, actor: &'ctx Actor) -> Result<Ctx<'ctx>, CommandBusError> {
        Ok(Self { pool, actor })
    }
    pub fn pool(&self) -> &PgPool {
        self.pool
    }

    pub fn actor(&self) -> &Actor {
        self.actor
    }
}
