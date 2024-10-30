mod bus;
pub mod commands;
mod error;

pub use bus::create_command_bus;
pub use bus::CommandBus;
pub use error::CommandBusError;
pub type CommandBusResult<T> = Result<T, CommandBusError>;
