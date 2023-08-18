use super::{
    commands::CommandType,
    handlers::{
        business_unit::{
            create_business_unit::CreateBusinessUnitError,
            delete_business_unit::DeleteBusinessUnitCommandHandlerError,
            update_business_unit::UpdateBusinessUnitError,
        },
        channel::{
            create_channel::CreateChannelCommandHandlerError,
            delete_channel::DeleteChannelCommandHandlerError,
            update_channel::UpdateChannelCommandHandlerError,
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
    CreateBusinessUnit(#[from] CreateBusinessUnitError),

    #[error(transparent)]
    UpdateBusinessUnit(#[from] UpdateBusinessUnitError),

    #[error(transparent)]
    DeleteBusinessUnit(#[from] DeleteBusinessUnitCommandHandlerError),

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

    #[error(transparent)]
    CreateChannel(#[from] CreateChannelCommandHandlerError),

    #[error(transparent)]
    UpdateChannel(#[from] UpdateChannelCommandHandlerError),

    #[error(transparent)]
    DeleteChannel(#[from] DeleteChannelCommandHandlerError),
}
