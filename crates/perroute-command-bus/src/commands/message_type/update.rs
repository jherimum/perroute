use crate::{
    bus::{Command, CommandBusContext, CommandHandler},
    CommandBusResult,
};
use bon::Builder;
use perroute_commons::types::{vars::Vars, Name, Schema};
use perroute_storage::{models::message_type::MessageType, repository::TransactedRepository};

#[derive(Debug, thiserror::Error)]
pub enum UpdateMessageTypeCommandError {}

#[derive(Debug, Clone, Builder)]
pub struct UpdateMessageTypeCommand {
    name: Name,
    vars: Option<Vars>,
    schema: Schema,
    enabled: bool,
}

impl Command for UpdateMessageTypeCommand {}

pub struct UpdateMessageTypeCommandHandler;

impl CommandHandler for UpdateMessageTypeCommandHandler {
    type Command = UpdateMessageTypeCommand;
    type Output = MessageType;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<'_, R>,
    ) -> CommandBusResult<Self::Output> {
        todo!()
    }
}
