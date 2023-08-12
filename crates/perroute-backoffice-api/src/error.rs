use actix_web::ResponseError;
use perroute_commons::{
    rest::RestError,
    types::{id::Id, json_schema::JsonSchemaError},
};
use perroute_cqrs::{command_bus::error::CommandBusError, query_bus::error::QueryBusError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    JsonChema(#[from] JsonSchemaError),

    #[error("Channel {0} not found")]
    ChannelNotFound(Id),

    #[error("ApiKey {0} not found")]
    ApiKeyNotFound(Id),

    #[error("Message type {0} not found")]
    MessageTypeNotFound(Id),

    #[error("Schema {0} not found")]
    SchemaNotFound(Id),

    #[error("Template {0} not found")]
    TemplateNotFound(Id),

    #[error(transparent)]
    CommandBus(#[from] CommandBusError),

    #[error(transparent)]
    QueryBus(#[from] QueryBusError),

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),

    #[error("{0}")]
    Const(&'static str),
}

impl From<&ApiError> for RestError {
    fn from(value: &ApiError) -> Self {
        match value {
            ApiError::ChannelNotFound(_)
            | ApiError::MessageTypeNotFound(_)
            | ApiError::SchemaNotFound(_)
            | ApiError::TemplateNotFound(_) => Self::NotFound(value.to_string()),
            ApiError::CommandBus(CommandBusError::ExpectedError(message)) => {
                Self::UnprocessableEntity(message.to_string())
            }
            _ => Self::InternalServer,
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
