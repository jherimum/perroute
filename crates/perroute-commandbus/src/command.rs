pub mod business_unit;
pub mod channel;
pub mod connection;
pub mod message;
pub mod message_type;
pub mod route;
pub mod template;

use crate::{bus::Ctx, error::CommandBusError};
use perroute_commons::types::{actor::Actor, command_type::CommandType};
use std::fmt::Debug;

pub type CommandResult<T> = Result<T, CommandBusError>;

#[async_trait::async_trait]
pub trait Command: Send + Sync + Debug {
    type Output;

    fn command_type(&self) -> CommandType;

    async fn handle<'ctx>(&self, ctx: &mut Ctx<'ctx>) -> CommandResult<Self::Output>;

    fn supports(&self, actor: &Actor) -> bool;
}
