use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ErrorResponse {
    pub status: u16,
    pub message: String,
    pub detail: Option<String>,
}

impl From<anyhow::Error> for RestError {
    fn from(_: anyhow::Error) -> Self {
        RestError::InternalServer
    }
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum RestError {
    #[error("Not found")]
    NotFound(String),

    #[error("Internal Server Error")]
    InternalServer,

    #[error("Unprocessable Entity")]
    UnprocessableEntity(String),
}

impl IntoResponse for RestError {
    fn into_response(self) -> axum::response::Response {
        let response: Json<ErrorResponse> = Json(self.clone().into());
        match self {
            RestError::NotFound(_) => (StatusCode::NOT_FOUND, response),
            RestError::InternalServer => (StatusCode::INTERNAL_SERVER_ERROR, response),
            RestError::UnprocessableEntity(_) => (StatusCode::UNPROCESSABLE_ENTITY, response),
        }
        .into_response()
    }
}

impl From<RestError> for ErrorResponse {
    fn from(value: RestError) -> Self {
        let message = value.to_string();
        match value {
            RestError::NotFound(detail) => {
                ErrorResponse::new(StatusCode::NOT_FOUND, message, Some(detail))
            }
            RestError::InternalServer => {
                ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, message, None)
            }
            RestError::UnprocessableEntity(detail) => {
                ErrorResponse::new(StatusCode::UNPROCESSABLE_ENTITY, message, Some(detail))
            }
        }
    }
}

impl ErrorResponse {
    pub fn new(status: StatusCode, message: String, detail: Option<String>) -> Self {
        Self {
            status: status.into(),
            message,
            detail,
        }
    }
}
