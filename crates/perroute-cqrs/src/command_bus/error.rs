use super::{
    commands::CommandType,
    handlers::{
        channel::{
            create_channel::CreateChannelError, delete_channel::DeleteChannelError,
            update_channel::UpdateChannelError,
        },
        message::{
            create_message::CreateMessageCommandError,
            distribute_message::DistributeMessageCommandHandlerError,
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

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),

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

    #[error(transparent)]
    CreateMessage(#[from] CreateMessageCommandError),

    #[error(transparent)]
    DistributeMessage(#[from] DistributeMessageCommandHandlerError),
}
