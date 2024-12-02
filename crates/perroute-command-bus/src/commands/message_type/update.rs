use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::{message_type::PayloadExamplesInput, Command},
    CommandBusError,
};
use bon::Builder;
use perroute_commons::types::{id::Id, name::Name, schema::Schema, vars::Vars, Payload};
use perroute_storage::{
    models::message_type::{MessageType, PayloadExample},
    repository::{
        message_types::{MessageTypeRepository, PayloadExampleRepository},
        TransactedRepository,
    },
};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum UpdateMessageTypeCommandError {
    #[error("Message type not found")]
    NotFound,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct UpdateMessageTypeCommand {
    id: Id,
    name: Name,
    vars: Vars,
    schema: Schema,
    enabled: bool,
    payload_examples: Vec<(Name, Payload)>,
}

impl Command for UpdateMessageTypeCommand {
    fn event_type(&self) -> perroute_commons::events::EventType {
        perroute_commons::events::EventType::MessageTypeUpdated
    }

    fn entity_id(&self) -> &Id {
        &self.id
    }
}

pub struct UpdateMessageTypeCommandHandler;

impl CommandHandler for UpdateMessageTypeCommandHandler {
    type Command = UpdateMessageTypeCommand;
    type Output = (MessageType, Vec<PayloadExample>);

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &crate::commands::CommandWrapper<'_, Self::Command>,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        let message_type = MessageTypeRepository::find_by_id(ctx.repository(), &cmd.inner().id)
            .await?
            .ok_or(CommandBusError::from(
                UpdateMessageTypeCommandError::NotFound,
            ))?
            .set_enabled(cmd.inner().enabled)
            .set_name(cmd.inner().name.clone())
            .set_schema(cmd.inner().schema.clone())
            .set_updated_at(cmd.created_at().clone())
            .set_vars(cmd.inner().vars.clone());

        let message_type =
            MessageTypeRepository::update_message_type(ctx.repository(), message_type).await?;

        PayloadExampleRepository::delete_payload_examples(ctx.repository(), message_type.id())
            .await?;

        let examples: Vec<PayloadExample> =
            PayloadExamplesInput::new(message_type.id(), &cmd.inner().payload_examples).into();

        let examples =
            PayloadExampleRepository::save_payload_examples(ctx.repository(), &examples).await?;

        Ok((message_type.clone(), examples))
    }
}
