use crate::bus::{Command, CommandBusContext, CommandHandler, CommandHandlerResult};
use bon::Builder;
use perroute_commons::{
    commands::CommandType,
    types::{priority::Priority, Configuration},
};
use perroute_storage::{models::route::Route, repository::TransactedRepository};

#[derive(Debug, thiserror::Error)]
pub enum UpdateRouteCommandError {}

#[derive(Debug, Clone, Builder)]
pub struct UpdateRouteCommand {
    configuration: Configuration,
    priority: Priority,
    enabled: bool,
}

impl Command for UpdateRouteCommand {
    fn command_type(&self) -> CommandType {
        CommandType::UpdateRoute
    }
}

pub struct UpdateRouteCommandHandler;

impl CommandHandler for UpdateRouteCommandHandler {
    type Command = UpdateRouteCommand;
    type Output = Route;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        todo!()
    }
}
