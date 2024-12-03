use crate::{
    commands::{Command, CommandWrapper},
    CommandBusError, CommandBusResult,
};
use perroute_commons::{
    events::ApplicationEvent,
    types::{actor::Actor, Timestamp},
};
use perroute_storage::{
    models::event::DbEvent,
    repository::{events::EventRepository, Repository, TransactedRepository},
};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    future::Future,
    sync::Arc,
};
use tap::TapFallible;

#[derive(Debug)]
pub struct CommandHandlerOutput<O, E> {
    pub output: O,
    pub event: E,
}

impl<O, E: ApplicationEvent> CommandHandlerOutput<O, E> {
    pub fn new(output: O, event: E) -> Self {
        Self { output, event }
    }
}

pub type CommandHandlerResult<O, E> = Result<CommandHandlerOutput<O, E>, CommandBusError>;

pub trait CommandBus {
    fn execute<C, H, O>(&self, actor: &Actor, cmd: &C) -> impl Future<Output = CommandBusResult<O>>
    where
        C: Command + 'static,
        H: CommandHandler<Command = C, Output = O> + 'static;
}

pub trait CommandHandler {
    type Command: Command;
    type Output;
    type ApplicationEvent: ApplicationEvent;

    fn handle<R: TransactedRepository>(
        &self,
        cmd: &CommandWrapper<Self::Command>,
        ctx: &CommandBusContext<'_, R>,
    ) -> impl Future<Output = CommandHandlerResult<Self::Output, Self::ApplicationEvent>>;
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

    pub fn register<C, H>(mut self, handler: H) -> Self
    where
        C: Command + 'static,
        H: CommandHandler<Command = C> + 'static + Sync + Send,
    {
        self.handlers.insert(
            TypeId::of::<C>(),
            Arc::new(handler) as Arc<dyn Any + Send + Sync>,
        );

        self
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
    async fn execute<C, H, O>(&self, actor: &Actor, command: &C) -> CommandBusResult<O>
    where
        C: Command + 'static,
        H: CommandHandler<Command = C, Output = O> + 'static,
    {
        let handler = self
            .get_handler::<C, H, O>()
            .tap_err(|e| log::error!("Failed to get command handler: {e}"))?;
        let tx = self
            .repository
            .begin()
            .await
            .tap_err(|e| log::error!("Failed to begin transaction: {e}"))?;

        let ctx = CommandBusContext { repository: &tx };
        let created_at = &Timestamp::now();
        let command_wrapper = CommandWrapper::new(command, created_at, actor);

        match handler.handle(&command_wrapper, &ctx).await {
            Ok(output) => {
                EventRepository::save(
                    &tx,
                    DbEvent::try_from(output.event.to_event(actor, created_at))
                        .tap_err(|e| log::error!("Failed to convert event to DbEvent: {e}"))?,
                )
                .await
                .tap_err(|e| log::error!("Failed to persist event: {}", e))?;

                tx.commit()
                    .await
                    .tap_err(|e| log::error!("Failed to commit transaction: {e}"))?;

                Ok(output.output)
            }
            Err(e) => {
                tx.rollback()
                    .await
                    .tap_err(|e| log::error!("Failed to rollback transaction: {e}"))?;
                Err(e)
            }
        }
    }
}

pub struct CommandBusContext<'r, R> {
    repository: &'r R,
}

impl<R: Repository> CommandBusContext<'_, R> {
    pub fn repository(&self) -> &R {
        self.repository
    }
}
