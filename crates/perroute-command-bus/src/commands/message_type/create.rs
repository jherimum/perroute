use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::{Command},
    impl_command,
};
use perroute_commons::{
    events::MessageTypeCreatedEvent,
    types::{
        code::Code, id::Id, name::Name, schema::Schema, vars::Vars, Payload,
    },
};
use perroute_storage::{
    models::message_type::{MessageType, PayloadExample},
};

use super::PayloadExamplesInput;

#[derive(Debug, thiserror::Error)]
pub enum CreateMessageTypeCommandError {
    #[error("Message type already exists")]
    AlreadyExists,
}

impl_command!(CreateMessageTypeCommand, {
    message_type_id: Id,
    code: Code,
    name: Name,
    vars: Vars,
    schema: Schema,
    enabled: bool,
    payload_examples: Vec<(Name, Payload)>,
});

pub struct CreateMessageTypeCommandHandler;

impl CommandHandler for CreateMessageTypeCommandHandler {
    type Command = CreateMessageTypeCommand;
    type Output = (MessageType, Vec<PayloadExample>);
    type E = MessageTypeCreatedEvent;

    async fn handle<
        C: AsRef<perroute_storage::active_record::datasource::Connection>,
    >(
        &self,

        ctx: &CommandBusContext<'_, C, Self::Command>,
    ) -> CommandHandlerResult<Self::Output> {
        // let exists = MessageTypeRepository::exists_message_type(
        //     ctx.repository(),
        //     &MessageTypeQuery::ByCode(cmd.inner().code.clone()),
        // )
        // .await?;

        // if exists {
        //     return Err(CreateMessageTypeCommandError::AlreadyExists.into());
        // }

        // let message_type = MessageType::builder()
        //     .id(cmd.inner().message_type_id.clone())
        //     .code(cmd.inner().code.clone())
        //     .name(cmd.inner().name.clone())
        //     .vars(cmd.inner().vars.clone())
        //     .schema(cmd.inner().schema.clone())
        //     .enabled(cmd.inner().enabled)
        //     .created_at(cmd.created_at().clone())
        //     .updated_at(cmd.created_at().clone())
        //     .build();

        // let message_type = MessageTypeRepository::save_message_type(
        //     ctx.repository(),
        //     message_type,
        // )
        // .await?;

        // let examples: Vec<PayloadExample> = PayloadExamplesInput::new(
        //     message_type.id(),
        //     &cmd.inner().payload_examples,
        // )
        // .into();

        // let examples = PayloadExampleRepository::save_payload_examples(
        //     ctx.repository(),
        //     &examples,
        // )
        // .await?;

        // //Ok((message_type, examples))
        todo!()
    }
}
