use crate::bus::{Command, CommandBusContext, CommandHandler, CommandHandlerResult};
use bon::Builder;
use perroute_commons::{
    commands::CommandType,
    types::{id::Id, Payload, Recipient, Tags, Timestamp},
};
use perroute_connectors::types::dispatch_type::DispatchType;
use perroute_storage::{models::message::Message, repository::TransactedRepository};

#[derive(Debug, thiserror::Error)]
pub enum CreateMessageCommandError {}

#[derive(Debug, Clone, Builder)]
pub struct CreateMessageCommand {
    id: Id,
    pub message_type_id: Id,
    pub business_unit_id: Id,
    pub payload: Payload,
    pub dispatch_type: DispatchType,
    pub recipient: Recipient,
    pub scheduled_at: Option<Timestamp>,
    pub tags: Tags,
}

impl Command for CreateMessageCommand {
    fn command_type(&self) -> CommandType {
        CommandType::CreateMessage
    }
}

pub struct CreateMessageCommandHandler;

impl CommandHandler for CreateMessageCommandHandler {
    type Command = CreateMessageCommand;
    type Output = Message;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        todo!()
    }
}
