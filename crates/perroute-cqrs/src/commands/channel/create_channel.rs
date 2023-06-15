use crate::command_bus::{Command, CommandBusContext, CommandHandler};
use anyhow::Context;
use async_trait::async_trait;
use derive_new::new;
use perroute_commons::types::code::Code;
use perroute_storage::models::channel::Channel;
use serde::Serialize;
use std::ops::DerefMut;

#[derive(Debug, new, Serialize, Clone, PartialEq, Eq)]
pub struct CreateChannelCommand {
    code: Code,
    name: String,
}

impl Command for CreateChannelCommand {
    fn name(&self) -> &str {
        "create_channel"
    }
}

#[derive(Debug, new)]
pub struct Handler;

#[derive(thiserror::Error, Debug)]
pub enum CreateChannelError {
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),

    #[error("A channel with code {0} already exists")]
    CodeAlreadyExists(Code),
}

#[async_trait]
impl CommandHandler for Handler {
    type Command = CreateChannelCommand;

    async fn handle<'tx>(
        &self,
        ctx: &CommandBusContext<'tx>,
        cmd: Self::Command,
    ) -> Result<String, String> {
        if Channel::exists_by_code(ctx.pool(), &cmd.code)
            .await
            .with_context(|| {
                format!(
                    "Error while checking if channel with code {} exists",
                    cmd.code,
                )
            })
            .map_err(|_| "")?
        {
            //return Err(CreateChannelError::CodeAlreadyExists(cmd.code)).map(CommandBusError::from);
            todo!()
        }

        let channel = Channel::new(&cmd.code, &cmd.name)
            .save(ctx.tx().write().await.deref_mut())
            .await
            .unwrap();

        // Ok(Event::ChannelEvent(crate::commands::ChannelEvent::Created(
        //     channel.id().clone(),
        // )))
        Ok("".to_owned())
    }
}
