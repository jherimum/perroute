use crate::bus::{
    Command, CommandBusContext, CommandHandler, CommandHandlerOutput, CommandHandlerResult,
};
use bon::Builder;
use perroute_commons::{
    commands::CommandType,
    events::Event,
    types::{code::Code, id::Id, name::Name, schema::Schema, vars::Vars, Payload, Timestamp},
};
use perroute_storage::{
    models::message_type::{MessageType, PayloadExample},
    repository::{
        message_types::{MessageTypeQuery, MessageTypeRepository, PayloadExampleRepository},
        TransactedRepository,
    },
};
use serde::Serialize;

use super::PayloadExamplesInput;

#[derive(Debug, thiserror::Error)]
pub enum CreateMessageTypeCommandError {
    #[error("Message type already exists")]
    AlreadyExists,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct CreateMessageTypeCommand {
    #[builder(default)]
    id: Id,
    code: Code,
    name: Name,
    vars: Vars,
    schema: Schema,
    enabled: bool,
    payload_examples: Vec<(Name, Payload)>,
}

impl Command for CreateMessageTypeCommand {
    fn command_type(&self) -> CommandType {
        CommandType::CreateMessageType
    }
}

pub struct CreateMessageTypeCommandHandler;

impl CommandHandler for CreateMessageTypeCommandHandler {
    type Command = CreateMessageTypeCommand;
    type Output = (MessageType, Vec<PayloadExample>);

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        let exists = MessageTypeRepository::exists_message_type(
            ctx.repository(),
            &MessageTypeQuery::ByCode(cmd.code.clone()),
        )
        .await?;

        if exists {
            return Err(CreateMessageTypeCommandError::AlreadyExists.into());
        }

        let message_type = MessageType::builder()
            .id(cmd.id.clone())
            .code(cmd.code.clone())
            .name(cmd.name.clone())
            .vars(cmd.vars.clone())
            .schema(cmd.schema.clone())
            .enabled(cmd.enabled)
            .created_at(Timestamp::now())
            .updated_at(Timestamp::now())
            .build();

        let message_type =
            MessageTypeRepository::save_message_type(ctx.repository(), message_type).await?;

        let examples: Vec<PayloadExample> =
            PayloadExamplesInput::new(message_type.id(), &cmd.payload_examples).into();

        let examples =
            PayloadExampleRepository::save_payload_examples(ctx.repository(), &examples).await?;

        CommandHandlerOutput::new((message_type.clone(), examples))
            .with_event(Event::MessageTypeCreated(message_type.id().clone()))
            .ok()
    }
}
