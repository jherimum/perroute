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
use perroute_commons::types::actor::Actor;
use perroute_events::ApplicationEvent;
use perroute_storage::{
    models::event::Event,
    repository::{events::EventRepository, Repository, TransactedRepository},
};
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

pub type CommandHandlerResult<O, E> = Result<CommandHandlerOutput<O, E>, CommandBusError>;

pub struct CommandHandlerOutput<O, E> {
    output: O,
    event: Option<E>,
}

impl<O, E> CommandHandlerOutput<O, E> {
    pub fn new(output: O, event: Option<E>) -> Self {
        Self { output, event }
    }
}

pub trait Command {}

pub trait CommandBus {
    fn execute<C, H, O, E>(
        &self,
        actor: &Actor,
        cmd: &C,
    ) -> impl Future<Output = CommandBusResult<O>>
    where
        C: Command + 'static,
        E: ApplicationEvent + 'static,
        H: CommandHandler<Command = C, Output = O, Event = E> + 'static;
}

pub trait CommandHandler {
    type Command;
    type Output;
    type Event;

    fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<R>,
    ) -> impl Future<Output = CommandBusResult<CommandHandlerOutput<Self::Output, Self::Event>>>;
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
    async fn execute<C, H, O, E>(&self, actor: &Actor, cmd: &C) -> CommandBusResult<O>
    where
        C: Command + 'static,
        E: ApplicationEvent + 'static,
        H: CommandHandler<Command = C, Output = O, Event = E> + 'static,
    {
        let handler = self.get_handler::<C, H, O>()?;
        let tx = self.repository.begin().await?;
        let ctx = CommandBusContext {
            repository: tx.clone(),
            actor,
        };
        match handler.handle(cmd, ctx).await {
            Ok(output) => {
                if let Some(event) = output.event {
                    EventRepository::save(&tx, Event::from(event))
                        .await
                        .tap_err(|e| log::error!("Failed to persist event: {}", e))?;
                }
                tx.commit().await?;
                Ok(output.output)
            }
            Err(e) => {
                tx.rollback().await?;
                Err(e)
            }
        }
    }
}

pub struct CommandBusContext<'a, R: TransactedRepository> {
    repository: R,
    actor: &'a Actor,
}

impl<'a, R: TransactedRepository> CommandBusContext<'a, R> {
    pub fn repository(&self) -> &R {
        &self.repository
    }

    pub fn actor(&self) -> &'a Actor {
        self.actor
    }
}
