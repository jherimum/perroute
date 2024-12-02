use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::Command,
};
use bon::Builder;
use perroute_commons::types::{
    dispatch_type::DispatchType, id::Id, recipient::Recipient, Payload, Tags, Timestamp,
};
use perroute_storage::{models::message::Message, repository::TransactedRepository};

#[derive(Debug, thiserror::Error)]
pub enum CreateMessageCommandError {}

#[derive(Debug, Clone, Builder)]
pub struct CreateMessageCommand {
    pub id: Id,
    pub message_type_id: Id,
    pub business_unit_id: Id,
    pub payload: Payload,
    pub dispatch_type: DispatchType,
    pub recipient: Recipient,
    pub scheduled_at: Option<Timestamp>,
    pub tags: Tags,
}

impl Command for CreateMessageCommand {
    fn event_type(&self) -> perroute_commons::events::EventType {
        perroute_commons::events::EventType::MessageCreated
    }

    fn entity_id(&self) -> &Id {
        &self.id
    }
}

pub struct CreateMessageCommandHandler;

impl CommandHandler for CreateMessageCommandHandler {
    type Command = CreateMessageCommand;
    type Output = Message;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &crate::commands::CommandWrapper<'_, Self::Command>,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        todo!()
    }
}
