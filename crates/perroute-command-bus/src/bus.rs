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
use perroute_commons::events::Event;
use perroute_commons::{commands::CommandType, types::actor::Actor};
use perroute_storage::{
    models::{command_audit::command_audit_builder, event::DbEvent},
    repository::{
        command_audit::CommandAuditRepository, events::EventRepository, Repository,
        TransactedRepository,
    },
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

pub type CommandHandlerResult<O> = Result<CommandHandlerOutput<O>, CommandBusError>;

#[derive(Debug)]
pub struct CommandHandlerOutput<O> {
    output: O,
    event: Option<Event>,
}

impl<O> CommandHandlerOutput<O> {
    pub fn new(output: O) -> Self {
        Self {
            output,
            event: None,
        }
    }
    pub fn with_event(mut self, event: Event) -> Self {
        self.event = Some(event);
        self
    }

    pub fn event(&self) -> Option<&Event> {
        self.event.as_ref()
    }

    pub fn ok<E>(self) -> Result<Self, E> {
        Ok(self)
    }
}

pub trait Command {
    fn command_type(&self) -> CommandType;
}

pub trait CommandBus {
    fn execute<C, H, O>(&self, actor: &Actor, cmd: &C) -> impl Future<Output = CommandBusResult<O>>
    where
        C: Command + 'static + Serialize,
        H: CommandHandler<Command = C, Output = O> + 'static;
}

pub trait CommandHandler {
    type Command;
    type Output;

    fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<R>,
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
    async fn execute<C, H, O>(&self, actor: &Actor, cmd: &C) -> CommandBusResult<O>
    where
        C: Command + 'static + Serialize,
        H: CommandHandler<Command = C, Output = O> + 'static,
    {
        let handler = self.get_handler::<C, H, O>()?;
        let tx = self.repository.begin().await?;

        let command_audit = command_audit_builder()
            .actor(actor)
            .command_data(cmd)
            .command_type(&cmd.command_type())
            .call();

        CommandAuditRepository::save(&tx, command_audit).await?;

        let ctx = CommandBusContext {
            repository: tx.clone(),
            actor,
        };
        match handler.handle(cmd, ctx).await {
            Ok(output) => {
                if let Some(event) = output.event() {
                    EventRepository::save(&tx, DbEvent::from(event))
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
