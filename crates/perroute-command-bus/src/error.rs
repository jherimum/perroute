use crate::commands::{
    business_unit::{
        create::CreateBusinessUnitCommandError,
        delete::DeleteBusinessUnitCommandError,
        update::UpdateBusinessUnitCommandError,
    },
    channel::{
        create::CreateChannelCommandError, delete::DeleteChannelCommandError,
        update::UpdateChannelCommandError,
    },
    message_type::{
        create::CreateMessageTypeCommandError,
        update::UpdateMessageTypeCommandError,
    },
    route::{create::CreateRouteCommandError, update::UpdateRouteCommandError},
};

#[derive(Debug, thiserror::Error)]
pub enum CommandBusError {
    #[error("Active record error: {0}")]
    ActiveRecordError(
        #[from] perroute_storage::active_record::ActiveRecordError,
    ),

    #[error("Command handler not found for command: {0}")]
    CommandHandlerNotFound(String),

    #[error("Create business unit command error: {0}")]
    CreateBusinessUnitCommandError(#[from] CreateBusinessUnitCommandError),

    #[error("Update business unit command error: {0}")]
    UpdateBusinessUnitCommandError(#[from] UpdateBusinessUnitCommandError),

    #[error("Delete business unit command error: {0}")]
    DeleteBusinessUnitCommandError(#[from] DeleteBusinessUnitCommandError),

    #[error("Create message type command error: {0}")]
    CreateMessageTypeCommandError(#[from] CreateMessageTypeCommandError),

    #[error("Update message type command error: {0}")]
    UpdateMessageTypeCommandError(#[from] UpdateMessageTypeCommandError),

    #[error("Create channel command error: {0}")]
    CreateChannelCommandError(#[from] CreateChannelCommandError),

    #[error("Update channel command error: {0}")]
    UpdateChannelCommandError(#[from] UpdateChannelCommandError),

    #[error("Delete channel command error: {0}")]
    DeleteChannelCommandError(#[from] DeleteChannelCommandError),

    #[error("Create route command error: {0}")]
    CreateRouteCommandError(#[from] CreateRouteCommandError),

    #[error("Update route command error: {0}")]
    UpdateRouteCommandError(#[from] UpdateRouteCommandError),

    #[error("{0}")]
    GeneralError(#[from] serde_json::Error),
}
