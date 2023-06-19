use perroute_commons::{crypto::CryptoError, rest::RestError};
use perroute_cqrs::{command_bus::error::CommandBusError, query_bus::error::QueryBusError};

#[derive(Debug, thiserror::Error)]
pub enum PerrouteError {
    #[error(transparent)]
    Database(#[from] sqlx::Error),

    #[error(transparent)]
    Crypto(#[from] CryptoError),

    #[error(transparent)]
    Rest(#[from] RestError),

    #[error(transparent)]
    CommandBus(#[from] CommandBusError),

    #[error(transparent)]
    QueryBus(#[from] QueryBusError),
}
