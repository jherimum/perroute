use super::{bus::CommandBusContext, commands::Command, Result};
use std::fmt::Debug;

pub mod business_unit;
pub mod channel;
pub mod connection;
pub mod message;
pub mod message_type;
pub mod route;
pub mod template;

#[async_trait::async_trait]
pub trait CommandHandler: Send + Sync + Debug {
    type Command: Command;
    type Output: Debug;

    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext,
        cmd: Self::Command,
    ) -> Result<Self::Output>;
}
