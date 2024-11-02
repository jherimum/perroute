use perroute_storage::repository;

use crate::commands::{
    business_unit::{
        create::CreateBusinessUnitCommandError, update::UpdateBusinessUnitCommandError,
    },
    channel::create::CreateChannelCommandError,
    message_type::{create::CreateMessageTypeCommandError, update::UpdateMessageTypeCommandError},
};

#[derive(Debug, thiserror::Error)]
pub enum CommandBusError {
    #[error("Command handler not found for command: {0}")]
    CommandHandlerNotFound(String),

    #[error("Repository error: {0}")]
    Repository(#[from] repository::Error),

    #[error("Create business unit command error: {0}")]
    CreateBusinessUnitCommandError(#[from] CreateBusinessUnitCommandError),

    #[error("Update business unit command error: {0}")]
    UpdateBusinessUnitCommandError(#[from] UpdateBusinessUnitCommandError),

    #[error("Create message type command error: {0}")]
    CreateMessageTypeCommandError(#[from] CreateMessageTypeCommandError),

    #[error("Update message type command error: {0}")]
    UpdateMessageTypeCommandError(#[from] UpdateMessageTypeCommandError),

    #[error("Create channel command error: {0}")]
    CreateChannelCommandError(#[from] CreateChannelCommandError),
}
