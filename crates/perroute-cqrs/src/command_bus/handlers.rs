use super::{bus::CommandBusContext, commands::Command, error::CommandBusError};
use std::fmt::Debug;

pub mod channel;
pub mod message_type;
pub mod schema;
pub mod template;

#[async_trait::async_trait]
pub trait CommandHandler: Send + Sync + Debug {
    type Command: Command;
    type Output: Debug;

    async fn handle<'tx, 'a>(
        &self,
        ctx: &mut CommandBusContext<'tx, 'a>,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError>;
}
