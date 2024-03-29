use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ErrorResponse {
    pub status: u16,
    pub message: String,
    pub detail: Option<String>,
}

impl ResponseError for RestError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::InternalServer => StatusCode::INTERNAL_SERVER_ERROR,
            Self::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
        }
    }
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let response: ErrorResponse = self.clone().into();
        HttpResponse::build(self.status_code()).json(response)
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

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,
}

impl From<RestError> for ErrorResponse {
    fn from(value: RestError) -> Self {
        let message = value.to_string();
        match value {
            RestError::NotFound(detail) => Self::new(StatusCode::NOT_FOUND, message, Some(detail)),
            RestError::InternalServer => {
                Self::new(StatusCode::INTERNAL_SERVER_ERROR, message, None)
            }
            RestError::UnprocessableEntity(detail) => {
                Self::new(StatusCode::UNPROCESSABLE_ENTITY, message, Some(detail))
            }
            RestError::Unauthorized => Self::new(StatusCode::UNAUTHORIZED, message, None),
            RestError::Forbidden => Self::new(StatusCode::FORBIDDEN, message, None),
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
