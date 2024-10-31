use bon::Builder;
use perroute_commons::types::{vars::Vars, Code, Name, Schema};
use perroute_storage::{models::message_type::MessageType, repository::TransactedRepository};
use crate::{bus::{Command, CommandBusContext, CommandHandler}, CommandBusResult};


#[derive(Debug, thiserror::Error)]
pub enum CreateMessageTypeCommandError{
}

#[derive(Debug, Clone, Builder)]
pub struct CreateMessageTypeCommand {
    code: Code,
    name: Name,
    vars: Option<Vars>,
    schema: Schema,
    enabled: bool,
}

impl Command for CreateMessageTypeCommand {
}


pub struct CreateMessageTypeCommandHandler;

impl CommandHandler for CreateMessageTypeCommandHandler {
    type Command = CreateMessageTypeCommand;
    type Output = MessageType;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<'_, R>,
    ) -> CommandBusResult<Self::Output> {
        todo!()
    }

}



