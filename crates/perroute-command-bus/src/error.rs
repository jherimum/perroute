use perroute_storage::repository;

use crate::commands::business_unit::{
    create::CreateBusinessUnitCommandError, update::UpdateBusinessUnitCommandError,
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
}
