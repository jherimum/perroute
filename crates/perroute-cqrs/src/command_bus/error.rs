use super::{
    commands::CommandType,
    handlers::{
        channel::{
            create_channel::CreateChannelError, delete_channel::DeleteChannelError,
            update_channel::UpdateChannelError,
        },
        message_type::{
            create_message_type::CreateMessageTypeError,
            delete_message_type::DeleteMessageTypeError,
            update_message_type::UpdateMessageTypeError,
        },
        schema::create_schema::CreateSchemaError,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum CommandBusError {
    #[error("{0}")]
    ExpectedError(&'static str),

    #[error("Command handler not found for command {0}")]
    HandlerNotFound(CommandType),

    #[error("{0}")]
    UnexpectedError(&'static str),

    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),

    #[error(transparent)]
    CreateChannel(#[from] CreateChannelError),

    #[error(transparent)]
    UpdateChannel(#[from] UpdateChannelError),

    #[error(transparent)]
    DeleteChannel(#[from] DeleteChannelError),

    #[error(transparent)]
    DeleteMessageType(#[from] DeleteMessageTypeError),

    #[error(transparent)]
    UpdateMessageType(#[from] UpdateMessageTypeError),

    #[error(transparent)]
    CreateMessageType(#[from] CreateMessageTypeError),

    #[error(transparent)]
    CreateSchema(#[from] CreateSchemaError),
}
