use std::fmt::Debug;

use super::{bus::CommandBusContext, commands::Command, error::CommandBusError};

pub mod channel;
pub mod message_type;
pub mod message_type_version;

#[async_trait::async_trait]
pub trait CommandHandler: Send + Sync + Debug {
    type Command: Command;

    async fn handle<'tx, 'a>(
        &self,
        ctx: &mut CommandBusContext<'tx, 'a>,
        cmd: Self::Command,
    ) -> Result<(), CommandBusError>;
}
