use super::{bus::CommandBusContext, commands::Command, error::CommandBusError};
use perroute_commons::types::actor::Actor;
use std::fmt::Debug;

pub mod business_unit;
pub mod message;
pub mod message_type;
pub mod schema;
pub mod template;

#[async_trait::async_trait]
pub trait CommandHandler: Send + Sync + Debug {
    type Command: Command;
    type Output: Debug;

    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        actor: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError>;
}
