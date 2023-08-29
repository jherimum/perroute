pub mod bus;
pub mod error;
pub mod handlers;
pub mod queries;

pub type Result<T> = std::result::Result<T, error::QueryBusError>;
