use crate::{commands::Command, CommandBusError, CommandBusResult};
use perroute_commons::{
    events::ApplicationEvent,
    types::{actor::Actor, Timestamp},
};
use perroute_connectors::ProviderPluginRepository;
use perroute_storage::active_record::datasource::{
    Connection, DataSource, NonTransactionalDataSource,
};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    future::Future,
    sync::Arc,
};
use tap::TapFallible;

pub type CommandHandlerResult<O> = Result<O, CommandBusError>;

pub trait CommandBus {
    fn execute<C, H, O>(
        &self,
        actor: &Actor,
        cmd: &C,
    ) -> impl Future<Output = CommandBusResult<O>>
    where
        C: Command + 'static,
        H: CommandHandler<Command = C, Output = O> + 'static;
}

pub trait CommandHandler {
    type Command: Command;
    type Output;
    type E: ApplicationEvent;

    fn handle<C: AsRef<Connection>>(
        &self,
        ctx: &CommandBusContext<'_, C, Self::Command>,
    ) -> impl Future<Output = CommandBusResult<Self::Output>>;

    fn into_event(
        command: &Self::Command,
        output: &Self::Output,
    ) -> Option<Self::E> {
        None
    }
}

#[derive(Clone)]
pub struct DefaultCommandBus {
    handlers: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
    datasource: DataSource<NonTransactionalDataSource>,
    plugin_repository: ProviderPluginRepository,
}

impl DefaultCommandBus {
    pub fn new(
        datasource: DataSource<NonTransactionalDataSource>,
        plugin_repository: ProviderPluginRepository,
    ) -> Self {
        Self {
            handlers: HashMap::new(),
            datasource,
            plugin_repository,
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
            CommandBusError::CommandHandlerNotFound(
                std::any::type_name::<C>().to_string(),
            )
        })
    }
}

impl CommandBus for DefaultCommandBus {
    async fn execute<C, H, O>(
        &self,
        actor: &Actor,
        command: &C,
    ) -> CommandBusResult<O>
    where
        C: Command + 'static,
        H: CommandHandler<Command = C, Output = O> + 'static,
    {
        let handler = self
            .get_handler::<C, H, O>()
            .tap_err(|e| log::error!("Failed to get command handler: {e}"))?;

        let ds_tx = self.datasource.begin_transaction().await.unwrap();

        let ctx = CommandBusContext::new(
            command,
            &ds_tx,
            actor,
            &self.plugin_repository,
        );

        match handler.handle(&ctx).await {
            Ok(output) => {
                if let Some(event) = H::into_event(command, &output) {
                    //DbEvent::try_from(event.to_event(actor, created_at));
                    println!("Event: {:?}", event);
                }

                // EventRepository::save(
                //     &tx,
                //     DbEvent::try_from(output.event.to_event(actor, created_at))
                //         .tap_err(|e| {
                //             log::error!(
                //                 "Failed to convert event to DbEvent: {e}"
                //             )
                //         })?,
                // )
                // .await
                // .tap_err(|e| log::error!("Failed to persist event: {}", e))?;

                ds_tx.commit().await.tap_err(|e| {
                    log::error!("Failed to commit transaction: {e}")
                })?;

                Ok(output)
            }
            Err(e) => {
                ds_tx.rollback().await.tap_err(|e| {
                    log::error!("Failed to rollback transaction: {e}")
                })?;
                Err(e)
            }
        }
    }
}

pub struct CommandBusContext<'c, C, CMD> {
    conn: &'c C,
    timestamp: Timestamp,
    actor: &'c Actor,
    command: &'c CMD,
    plugin_repository: &'c ProviderPluginRepository,
}

impl<'c, C: AsRef<Connection>, CMD: Command> CommandBusContext<'c, C, CMD> {
    pub fn new(
        command: &'c CMD,
        conn: &'c C,
        actor: &'c Actor,
        plugin_repository: &'c ProviderPluginRepository,
    ) -> Self {
        Self {
            conn,
            timestamp: Timestamp::now(),
            actor,
            command,
            plugin_repository,
        }
    }
}

impl<C: AsRef<Connection>, CMD: Command> CommandBusContext<'_, C, CMD> {
    pub fn datasource(&self) -> &C {
        self.conn
    }

    pub fn timestamp(&self) -> &Timestamp {
        &self.timestamp
    }

    pub fn actor(&self) -> &Actor {
        self.actor
    }

    pub fn command(&self) -> &CMD {
        self.command
    }

    pub fn plugin_repository(&self) -> &ProviderPluginRepository {
        self.plugin_repository
    }
}
