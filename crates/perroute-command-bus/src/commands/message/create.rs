use crate::bus::{
    Command, CommandBusContext, CommandHandler, CommandHandlerResult, CommandWrapper,
};
use bon::Builder;
use perroute_commons::{
    commands::CommandType,
    types::{dispatch_type::DispatchType, id::Id, recipient::Recipient, Payload, Tags, Timestamp},
};
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
    type Output = Message;

    fn command_type(&self) -> CommandType {
        CommandType::CreateMessage
    }

    fn to_event(
        &self,
        created_at: &perroute_commons::types::Timestamp,
        actor: &perroute_commons::types::actor::Actor,
        output: &Self::Output,
    ) -> perroute_commons::events::Event {
        todo!()
    }
}

pub struct CreateMessageCommandHandler;

impl CommandHandler for CreateMessageCommandHandler {
    type Command = CreateMessageCommand;
    type Output = Message;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: CommandWrapper<'_, Self::Command>,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        todo!()
    }
}
