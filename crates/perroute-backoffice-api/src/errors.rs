use axum::response::IntoResponse;
use perroute_commons::{crypto::CryptoError, rest::RestError};
use perroute_cqrs::message_bus::MessageBusError;

#[derive(Debug, thiserror::Error)]
pub enum PerrouteError {
    #[error(transparent)]
    Database(#[from] sqlx::Error),

    #[error(transparent)]
    Crypto(#[from] CryptoError),

    #[error(transparent)]
    Rest(#[from] RestError),

    #[error(transparent)]
    MessageBus(MessageBusError),
}

impl IntoResponse for PerrouteError {
    fn into_response(self) -> axum::response::Response {
        match self {
            PerrouteError::Rest(e) => e,
            _ => RestError::InernalServer,
        }
        .into_response()
    }
}
