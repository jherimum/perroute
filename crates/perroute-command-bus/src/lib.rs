use perroute_storage::{
    models::{application::Application, user::User},
    repository::{self, Repository, TransactedRepository},
};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
    future::Future,
    sync::Arc,
};

pub fn create_command_bus<R: Repository + Clone>(repository: R) -> impl CommandBus + Clone {
    DefaultCommandBus::new(repository)
}

pub type CommandBusResult<T> = Result<T, CommandBusError>;

pub enum Actor {
    User(User),
    System,
    Application(Application),
}

#[derive(Debug, thiserror::Error)]
pub enum CommandBusError {
    #[error("Command handler not found for command: {0}")]
    CommandHandlerNotFound(String),

    #[error("Repository error: {0}")]
    Repository(#[from] repository::Error),
}

pub struct CommandBusContext<'a, R: TransactedRepository + Clone> {
    repository: R,
    actor: &'a Actor,
}

pub trait CommandHandler {
    type Command: Command;
    type Output: Debug;

    fn handle<R: TransactedRepository + Clone>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<R>,
    ) -> impl Future<Output = CommandBusResult<Self::Output>>;
}

pub trait Command {}

pub trait CommandBus {
    fn execute<C, H, O>(&self, actor: &Actor, cmd: &C) -> impl Future<Output = CommandBusResult<O>>
    where
        C: Command + 'static,
        H: CommandHandler<Command = C, Output = O> + 'static;
}

#[derive(Clone)]
pub struct DefaultCommandBus<R> {
    repository: R,
    handlers: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl<R: Repository> DefaultCommandBus<R> {
    pub fn new(repository: R) -> Self {
        Self {
            repository,
            handlers: HashMap::new(),
        }
    }

    fn get_handler<C, H, O>(&self) -> CommandBusResult<&H>
    where
        C: Command + 'static,
        H: CommandHandler<Command = C, Output = O> + 'static,
    {
        let handler = self.handlers.get(&TypeId::of::<C>());
        handler.and_then(|h| h.downcast_ref::<H>()).ok_or_else(|| {
            CommandBusError::CommandHandlerNotFound(std::any::type_name::<C>().to_string())
        })
    }
}

impl<R: Repository> CommandBus for DefaultCommandBus<R> {
    async fn execute<C, H, O>(&self, actor: &Actor, cmd: &C) -> CommandBusResult<O>
    where
        C: Command + 'static,
        H: CommandHandler<Command = C, Output = O> + 'static,
    {
        let handler = self.get_handler::<C, H, O>()?;
        let tx = self.repository.begin().await?;
        let ctx = CommandBusContext {
            repository: tx.clone(),
            actor,
        };
        match handler.handle(cmd, ctx).await {
            Ok(output) => {
                tx.commit().await?;
                Ok(output)
            }
            Err(e) => {
                tx.rollback().await?;
                Err(e)
            }
        }
    }
}
