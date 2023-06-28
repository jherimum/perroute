use actix_web::ResponseError;
use perroute_commons::rest::RestError;
use perroute_cqrs::{command_bus::error::CommandBusError, query_bus::error::QueryBusError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    CommandBus(#[from] CommandBusError),

    #[error(transparent)]
    QueryBus(#[from] QueryBusError),
}

impl From<&ApiError> for RestError {
    fn from(value: &ApiError) -> Self {
        RestError::InternalServer
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
