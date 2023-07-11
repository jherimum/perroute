use actix_web::ResponseError;
use perroute_commons::{
    rest::RestError,
    types::{id::Id, json_schema::JsonSchemaError},
};
use perroute_cqrs::{
    command_bus::{
        commands::{
            CreateChannelCommandBuilderError, CreateMessageTypeCommandBuilderError,
            DeleteChannelCommandBuilderError, DeleteMessageTypeCommandBuilderError,
            UpdateChannelCommandBuilderError, UpdateMessageTypeCommandBuilderError,
        },
        error::CommandBusError,
    },
    query_bus::{
        error::QueryBusError,
        queries::{
            FindChannelQueryBuilderError, FindMessageTypeQueryBuilderError,
            QueryChannelsQueryBuilderError, QueryMessageTypesQueryBuilderError,
        },
    },
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    FindMessageTypeQueryBuilder(#[from] FindMessageTypeQueryBuilderError),

    #[error(transparent)]
    DeleteMessageTypeCommandBuilder(#[from] DeleteMessageTypeCommandBuilderError),

    #[error(transparent)]
    UpdateMessageTypeCommandBuilder(#[from] UpdateMessageTypeCommandBuilderError),

    #[error(transparent)]
    CreateMessageTypeCommandBuilder(#[from] CreateMessageTypeCommandBuilderError),

    #[error(transparent)]
    QueryMessageTypesQueryBuilder(#[from] QueryMessageTypesQueryBuilderError),

    #[error(transparent)]
    FindChannelQueryBuilder(#[from] FindChannelQueryBuilderError),

    #[error(transparent)]
    DeleteChannelCommandBuilder(#[from] DeleteChannelCommandBuilderError),

    #[error(transparent)]
    UpdateChannelCommandBuilder(#[from] UpdateChannelCommandBuilderError),

    #[error(transparent)]
    QueryChannelsQueryBuilder(#[from] QueryChannelsQueryBuilderError),

    #[error(transparent)]
    CreateChannelCommandBuilder(#[from] CreateChannelCommandBuilderError),

    #[error(transparent)]
    JsonChema(#[from] JsonSchemaError),

    #[error("Channel {0} not found")]
    ChannelNotFound(Id),

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
            ApiError::ChannelNotFound(_) => RestError::NotFound(value.to_string()),
            ApiError::MessageTypeNotFound(_) => RestError::NotFound(value.to_string()),
            ApiError::SchemaNotFound(_) => RestError::NotFound(value.to_string()),
            ApiError::TemplateNotFound(_) => RestError::NotFound(value.to_string()),
            ApiError::CommandBus(CommandBusError::ExpectedError(message)) => {
                RestError::UnprocessableEntity(message.to_string())
            }
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
