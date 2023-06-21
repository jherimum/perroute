use super::{
    commands::CommandType,
    handlers::channel::{
        create_channel::CreateChannelError, delete_channel::DeleteChannelError,
        update_channel::UpdateChannelError,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum CommandBusError {
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
}
