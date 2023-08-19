use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ErrorResponse {
    pub status: u16,
    pub message: String,
    pub detail: Option<String>,
    pub errors: Option<HashMap<String, String>>,
}

impl ResponseError for RestError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        self.clone().into()
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

    #[error("BadRequest")]
    BadRequest(Option<HashMap<String, String>>),
}

impl From<RestError> for StatusCode {
    fn from(value: RestError) -> Self {
        match value {
            RestError::NotFound(_) => StatusCode::NOT_FOUND,
            RestError::InternalServer => StatusCode::INTERNAL_SERVER_ERROR,
            RestError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            RestError::Unauthorized => StatusCode::UNAUTHORIZED,
            RestError::Forbidden => StatusCode::FORBIDDEN,
            RestError::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl From<RestError> for ErrorResponse {
    fn from(value: RestError) -> Self {
        let message = value.to_string();
        match value {
            RestError::NotFound(detail) => {
                Self::new(StatusCode::NOT_FOUND, message, Some(detail), None)
            }
            RestError::InternalServer => {
                Self::new(StatusCode::INTERNAL_SERVER_ERROR, message, None, None)
            }
            RestError::UnprocessableEntity(detail) => Self::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                message,
                Some(detail),
                None,
            ),
            RestError::Unauthorized => Self::new(StatusCode::UNAUTHORIZED, message, None, None),
            RestError::Forbidden => Self::new(StatusCode::FORBIDDEN, message, None, None),
            RestError::BadRequest(e) => Self::new(
                StatusCode::BAD_REQUEST,
                message,
                Some("Invalid params".to_owned()),
                e,
            ),
        }
    }
}

impl ErrorResponse {
    pub fn new(
        status: StatusCode,
        message: String,
        detail: Option<String>,
        errors: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            status: status.into(),
            message,
            detail,
            errors,
        }
    }
}
