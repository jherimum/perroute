use crate::{
    bus::{Command, CommandBusContext, CommandHandler},
    commands::message_type::PayloadExamplesInput,
    CommandBusError, CommandBusResult,
};
use bon::Builder;
use perroute_commons::types::{id::Id, name::Name, schema::Schema, vars::Vars, Payload, Timestamp};
use perroute_storage::{
    models::message_type::{MessageType, PayloadExample},
    repository::{
        message_types::{MessageTypeRepository, PayloadExampleRepository},
        TransactedRepository,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum UpdateMessageTypeCommandError {
    #[error("Message type not found")]
    NotFound,
}

#[derive(Debug, Clone, Builder)]
pub struct UpdateMessageTypeCommand {
    id: Id,
    name: Name,
    vars: Option<Vars>,
    schema: Schema,
    enabled: bool,
    payload_examples: Vec<(Name, Payload)>,
}

impl Command for UpdateMessageTypeCommand {}

pub struct UpdateMessageTypeCommandHandler;

impl CommandHandler for UpdateMessageTypeCommandHandler {
    type Command = UpdateMessageTypeCommand;
    type Output = (MessageType, Vec<PayloadExample>);

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<'_, R>,
    ) -> CommandBusResult<Self::Output> {
        let message_type = MessageTypeRepository::find_message_type(ctx.repository(), &cmd.id)
            .await?
            .ok_or(CommandBusError::from(
                UpdateMessageTypeCommandError::NotFound,
            ))?
            .set_enabled(cmd.enabled)
            .set_name(cmd.name.clone())
            .set_schema(cmd.schema.clone())
            .set_updated_at(Timestamp::now())
            .set_vars(cmd.vars.clone());

        let message_type =
            MessageTypeRepository::update_message_type(ctx.repository(), message_type).await?;

        PayloadExampleRepository::delete_payload_examples(ctx.repository(), message_type.id())
            .await?;

        let examples: Vec<PayloadExample> =
            PayloadExamplesInput::new(&message_type.id(), &cmd.payload_examples).into();

        let examples =
            PayloadExampleRepository::save_payload_examples(ctx.repository(), &examples).await?;

        Ok((message_type, examples))
    }
}
