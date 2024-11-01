use crate::{
    bus::{Command, CommandBusContext, CommandHandler},
    CommandBusResult,
};
use bon::Builder;
use perroute_commons::types::{Configuration, Priority};
use perroute_storage::{models::route::Route, repository::TransactedRepository};

#[derive(Debug, thiserror::Error)]
pub enum UpdateRouteCommandError {}

#[derive(Debug, Clone, Builder)]
pub struct UpdateRouteCommand {
    configuration: Configuration,
    priority: Priority,
    enabled: bool,
}

impl Command for UpdateRouteCommand {}

pub struct UpdateRouteCommandHandler;

impl CommandHandler for UpdateRouteCommandHandler {
    type Command = UpdateRouteCommand;
    type Output = Route;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<'_, R>,
    ) -> CommandBusResult<Self::Output> {
        todo!()
    }
}
