use crate::{
    bus::{Command, CommandBusContext, CommandHandler},
    CommandBusResult,
};
use bon::Builder;
use perroute_commons::types::{id::Id, Configuration, DispatchType, Name, ProviderId};
use perroute_storage::{models::channel::Channel, repository::TransactedRepository};

#[derive(Debug, thiserror::Error)]
pub enum CreateChannelCommandError {}

#[derive(Debug, Clone, Builder)]
pub struct CreateChannelCommand {
    business_unit_id: Id,
    name: Name,
    provider_id: ProviderId,
    dispatch_type: DispatchType,
    configuration: Configuration,
    enabled: bool,
}

impl Command for CreateChannelCommand {}

pub struct CreateChannelCommandHandler;

impl CommandHandler for CreateChannelCommandHandler {
    type Command = CreateChannelCommand;
    type Output = Channel;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<'_, R>,
    ) -> CommandBusResult<Self::Output> {
        todo!()
    }
}
