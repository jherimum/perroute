use crate::bus::{
    Command, CommandBusContext, CommandHandler, CommandHandlerOutput, CommandHandlerResult,
};
use bon::Builder;
use perroute_commons::types::{
    code::Code, id::Id, name::Name, schema::Schema, vars::Vars, Payload, Timestamp,
};
use perroute_storage::{
    models::message_type::{MessageType, PayloadExample},
    repository::{
        message_types::{MessageTypeQuery, MessageTypeRepository, PayloadExampleRepository},
        TransactedRepository,
    },
};

use super::PayloadExamplesInput;

#[derive(Debug, thiserror::Error)]
pub enum CreateMessageTypeCommandError {
    #[error("Message type already exists")]
    AlreadyExists,
}

#[derive(Debug, Clone, Builder)]
pub struct CreateMessageTypeCommand {
    code: Code,
    name: Name,
    vars: Option<Vars>,
    schema: Schema,
    enabled: bool,
    payload_examples: Vec<(Name, Payload)>,
}

impl Command for CreateMessageTypeCommand {}

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
            .id(Id::new())
            .code(cmd.code.clone())
            .name(cmd.name.clone())
            .maybe_vars(cmd.vars.clone())
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

        Ok(CommandHandlerOutput::new((message_type, examples), None))
    }
}
