pub mod business_unit;
pub mod channel;
pub mod connection;
pub mod message;
pub mod message_type;

use crate::{bus::Ctx, error::CommandBusError};
use perroute_commons::types::{actor::Actor, command_type::CommandType};
use std::fmt::Debug;

#[async_trait::async_trait]
pub trait Command: Send + Sync + Debug {
    type Output;

    fn command_type(&self) -> CommandType;

    async fn handle<'ctx>(&self, ctx: &mut Ctx<'ctx>) -> Result<Self::Output, CommandBusError>;

    fn supports(&self, actor: &Actor) -> bool;
}
