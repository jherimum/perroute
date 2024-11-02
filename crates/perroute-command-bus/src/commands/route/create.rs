use crate::{
    bus::{Command, CommandBusContext, CommandHandler, CommandHandlerResult},
    CommandBusResult,
};
use bon::Builder;
use perroute_commons::types::{id::Id, priority::Priority, Configuration};
use perroute_storage::{models::route::Route, repository::TransactedRepository};

#[derive(Debug, thiserror::Error)]
pub enum CreateRouteCommandError {}

#[derive(Debug, Clone, Builder)]
pub struct CreateRouteCommand {
    business_unit_id: Id,
    channel_id: Id,
    message_type_id: Id,
    configuration: Configuration,
    priority: Priority,
    enabled: bool,
}

impl Command for CreateRouteCommand {}

pub struct CreateRouteCommandHandler;

impl CommandHandler for CreateRouteCommandHandler {
    type Command = CreateRouteCommand;
    type Output = Route;
    type Event = ();

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output, Self::Event> {
        todo!()
    }
}
