pub mod bus;
pub mod commands;
pub mod error;
pub mod handlers;

pub type Result<T> = std::result::Result<T, error::CommandBusError>;
