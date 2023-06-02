use axum::response::IntoResponse;
use omni_commons::{crypto::CryptoError, rest::RestError};
use omni_cqrs::message_bus::MessageBusError;

#[derive(Debug, thiserror::Error)]
pub enum OmniMessageError {
    #[error(transparent)]
    Database(#[from] sqlx::Error),

    #[error(transparent)]
    Crypto(#[from] CryptoError),

    #[error(transparent)]
    Rest(#[from] RestError),

    #[error(transparent)]
    MessageBus(MessageBusError),
}

impl IntoResponse for OmniMessageError {
    fn into_response(self) -> axum::response::Response {
        match self {
            OmniMessageError::Rest(e) => e,
            _ => RestError::InernalServer,
        }
        .into_response()
    }
}
