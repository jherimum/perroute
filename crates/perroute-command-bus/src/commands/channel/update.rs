use crate::{
    bus::{Command, CommandBusContext, CommandHandler},
    CommandBusResult,
};
use bon::Builder;
use perroute_commons::types::{Configuration, Name};
use perroute_storage::{models::channel::Channel, repository::TransactedRepository};

#[derive(Debug, thiserror::Error)]
pub enum UpdateChannelCommandError {}

#[derive(Debug, Clone, Builder)]
pub struct UpdateChannelCommand {
    name: Name,
    configuration: Configuration,
    enabled: bool,
}

impl Command for UpdateChannelCommand {}

pub struct UpdateChannelCommandHandler;

impl CommandHandler for UpdateChannelCommandHandler {
    type Command = UpdateChannelCommand;
    type Output = Channel;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<'_, R>,
    ) -> CommandBusResult<Self::Output> {
        todo!()
    }
}
