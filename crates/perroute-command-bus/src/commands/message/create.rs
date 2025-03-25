use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::{Command},
    impl_command,
};
use perroute_commons::{
    events::MessageCreatedEvent,
    types::{
        dispatch_type::DispatchType, id::Id, recipient::Recipient, Payload,
        Tags, Timestamp,
    },
};
use perroute_storage::{models::message::Message};

#[derive(Debug, thiserror::Error)]
pub enum CreateMessageCommandError {}

impl_command!(CreateMessageCommand, {
     message_id: Id,
     message_type_id: Id,
     business_unit_id: Id,
     payload: Payload,
     dispatch_type: DispatchType,
     recipient: Recipient,
     scheduled_at: Option<Timestamp>,
     tags: Tags,
});

pub struct CreateMessageCommandHandler;

impl CommandHandler for CreateMessageCommandHandler {
    type Command = CreateMessageCommand;
    type Output = Message;
    type E = MessageCreatedEvent;

    async fn handle<
        C: AsRef<perroute_storage::active_record::datasource::Connection>,
    >(
        &self,

        ctx: &CommandBusContext<'_, C, Self::Command>,
    ) -> CommandHandlerResult<Self::Output> {
        todo!()
    }
}
