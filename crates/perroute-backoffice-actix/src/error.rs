use actix_web::ResponseError;
use perroute_commons::{rest::RestError, types::code::Code};
use perroute_cqrs::{command_bus::error::CommandBusError, query_bus::error::QueryBusError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Channel {0} not found")]
    ChannelNotFound(Code),

    #[error(transparent)]
    CommandBus(#[from] CommandBusError),

    #[error(transparent)]
    QueryBus(#[from] QueryBusError),

    #[error(transparent)]
    Rest(#[from] RestError),

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),

    #[error("{0}")]
    Const(&'static str),
}

impl From<&ApiError> for RestError {
    fn from(value: &ApiError) -> Self {
        match value {
            ApiError::Rest(e) => e.clone(),
            _ => RestError::InternalServer,
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        let rest: RestError = self.into();
        ResponseError::status_code(&rest)
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let rest: RestError = self.into();
        ResponseError::error_response(&rest)
    }
}
