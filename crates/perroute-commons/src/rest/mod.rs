use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize, __private::de};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ErrorResponse {
    pub status: u16,
    pub message: String,
    pub detail: Option<String>,
}

#[derive(Debug, thiserror::Error)]
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
        match self {
            RestError::NotFound(_) => (StatusCode::NOT_FOUND, Json(self.as_error_response())),
            RestError::InternalServer => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(self.as_error_response()),
            ),
            RestError::UnprocessableEntity(_) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(self.as_error_response()),
            ),
        }
        .into_response()
    }
}

impl RestError {
    pub fn as_error_response(self) -> ErrorResponse {
        let message = self.to_string();
        match self {
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
