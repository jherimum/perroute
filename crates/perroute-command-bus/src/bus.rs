use crate::{
    commands::{
        business_unit::{
            create::CreateBusinessUnitCommandHandler, delete::DeleteBusinessUnitCommandHandler,
            update::UpdateBusinessUnitCommandHandler,
        },
        channel::{
            create::CreateChannelCommandHandler, delete::DeleteChannelCommandHandler,
            update::UpdateChannelCommandHandler,
        },
        message::create::CreateMessageCommandHandler,
        message_type::{
            create::CreateMessageTypeCommandHandler, update::UpdateMessageTypeCommandHandler,
        },
        route::{
            create::CreateRouteCommandHandler, delete::DeleteRouteCommandHandler,
            update::UpdateRouteCommandHandler,
        },
        template_assignment::{
            create::CreateTemplateAssignmentCommandHandler,
            delete::DeleteTemplateAssignmentCommandHandler,
            update::UpdateTemplateAssignmentCommandHandler,
        },
    },
    CommandBusError, CommandBusResult,
};
use perroute_commons::{
    commands::CommandType,
    events::Event,
    types::{actor::Actor, Timestamp},
};
use perroute_storage::{
    models::event::DbEvent,
    repository::{events::EventRepository, Repository, TransactedRepository},
};
use serde::Serialize;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    future::Future,
    sync::Arc,
};
use tap::TapFallible;

pub fn create_command_bus<R: Repository + Clone>(repository: R) -> impl CommandBus + Clone {
    DefaultCommandBus::new(repository)
        .register(CreateBusinessUnitCommandHandler)
        .register(DeleteBusinessUnitCommandHandler)
        .register(UpdateBusinessUnitCommandHandler)
        .register(CreateMessageTypeCommandHandler)
        .register(UpdateMessageTypeCommandHandler)
        .register(DeleteBusinessUnitCommandHandler)
        .register(CreateRouteCommandHandler)
        .register(UpdateRouteCommandHandler)
        .register(DeleteRouteCommandHandler)
        .register(CreateChannelCommandHandler)
        .register(UpdateChannelCommandHandler)
        .register(DeleteChannelCommandHandler)
        .register(CreateTemplateAssignmentCommandHandler)
        .register(UpdateTemplateAssignmentCommandHandler)
        .register(DeleteTemplateAssignmentCommandHandler)
        .register(CreateMessageCommandHandler)
}

pub type CommandHandlerResult<O> = Result<O, CommandBusError>;

pub struct CommandWrapper<'c, C: Command> {
    command: &'c C,
    created_at: &'c Timestamp,
    actor: &'c Actor,
}

impl<C: Command> CommandWrapper<'_, C> {
    pub fn inner(&self) -> &C {
        self.command
    }

    pub fn created_at(&self) -> &Timestamp {
        &self.created_at
    }

    pub fn actor(&self) -> &Actor {
        &self.actor
    }
}

pub trait Command {
    type Output;

    fn command_type(&self) -> CommandType;
    fn to_event(&self, created_at: &Timestamp, actor: &Actor, output: &Self::Output) -> Event;
}

pub trait CommandBus {
    fn execute<C, H, O>(&self, actor: &Actor, cmd: &C) -> impl Future<Output = CommandBusResult<O>>
    where
        C: Command<Output = O> + 'static + Serialize,
        H: CommandHandler<Command = C, Output = O> + 'static;
}

pub trait CommandHandler {
    type Command: Command;
    type Output;

    fn handle<R: TransactedRepository>(
        &self,
        cmd: CommandWrapper<Self::Command>,
        ctx: &CommandBusContext<'_, R>,
    ) -> impl Future<Output = CommandHandlerResult<Self::Output>>;
}

#[derive(Clone)]
struct DefaultCommandBus<R> {
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
        C: Command<Output = O> + 'static + Serialize,
        H: CommandHandler<Command = C, Output = C::Output> + 'static,
    {
        let handler = self.get_handler::<C, H, O>()?;
        let tx = self.repository.begin().await?;
        let ctx = CommandBusContext { repository: &tx };
        let created_at = &Timestamp::now();
        match handler
            .handle(
                CommandWrapper {
                    command,
                    created_at,
                    actor,
                },
                &ctx,
            )
            .await
        {
            Ok(output) => {
                EventRepository::save(
                    &tx,
                    DbEvent::try_from(command.to_event(&created_at, &actor, &output)).unwrap(),
                )
                .await
                .tap_err(|e| log::error!("Failed to persist event: {}", e))?;

                tx.commit()
                    .await
                    .tap_err(|e| log::error!("Failed to commit transaction: {e}"))?;

                Ok(output)
            }
            Err(e) => {
                tx.rollback().await?;
                Err(e)
            }
        }
    }
}

pub struct CommandBusContext<'r, R: TransactedRepository> {
    repository: &'r R,
}

impl<'r, R: TransactedRepository> CommandBusContext<'_, R> {
    pub fn repository(&self) -> &R {
        &self.repository
    }
}
