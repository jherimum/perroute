use crate::bus::{
    Command, CommandBusContext, CommandHandler, CommandHandlerResult, CommandWrapper,
};
use bon::Builder;
use perroute_commons::{
    commands::CommandType,
    types::{code::Code, id::Id, name::Name, schema::Schema, vars::Vars, Payload},
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
    type Output = (MessageType, Vec<PayloadExample>);

    fn command_type(&self) -> CommandType {
        CommandType::CreateMessageType
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

pub struct CreateMessageTypeCommandHandler;

impl CommandHandler for CreateMessageTypeCommandHandler {
    type Command = CreateMessageTypeCommand;
    type Output = (MessageType, Vec<PayloadExample>);

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: CommandWrapper<'_, Self::Command>,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        let exists = MessageTypeRepository::exists_message_type(
            ctx.repository(),
            &MessageTypeQuery::ByCode(cmd.inner().code.clone()),
        )
        .await?;

        if exists {
            return Err(CreateMessageTypeCommandError::AlreadyExists.into());
        }

        let message_type = MessageType::builder()
            .id(cmd.inner().id.clone())
            .code(cmd.inner().code.clone())
            .name(cmd.inner().name.clone())
            .vars(cmd.inner().vars.clone())
            .schema(cmd.inner().schema.clone())
            .enabled(cmd.inner().enabled)
            .created_at(cmd.created_at().clone())
            .updated_at(cmd.created_at().clone())
            .build();

        let message_type =
            MessageTypeRepository::save_message_type(ctx.repository(), message_type).await?;

        let examples: Vec<PayloadExample> =
            PayloadExamplesInput::new(message_type.id(), &cmd.inner().payload_examples).into();

        let examples =
            PayloadExampleRepository::save_payload_examples(ctx.repository(), &examples).await?;

        Ok((message_type, examples))
    }
}
