use perroute_storage::error::StorageError;

use crate::command::{
    business_unit::{
        create_business_unit::CreateBusinessUnitError,
        delete_business_unit::DeleteBusinessUnitError,
        update_business_unit::UpdateBusinessUnitError,
    },
    channel::{
        create_channel::CreateChannelError, delete_channel::DeleteChannelError,
        update_channel::UpdateChannelError,
    },
    connection::{
        create_connection::CreateConnectionError, delete_connection::DeleteConnectionError,
        update_connection::UpdateConnectionError,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum CommandBusError {
    #[error("")]
    ActorNotSupported,

    #[error(transparent)]
    CreateBusinessUnit(#[from] CreateBusinessUnitError),

    #[error(transparent)]
    UpdateBusinessUnit(#[from] UpdateBusinessUnitError),

    #[error(transparent)]
    DeleteBusinessUnit(#[from] DeleteBusinessUnitError),

    #[error(transparent)]
    CreateChannel(#[from] CreateChannelError),

    #[error(transparent)]
    DeleteChannel(#[from] DeleteChannelError),

    #[error(transparent)]
    UpdateChannel(#[from] UpdateChannelError),

    #[error(transparent)]
    CreateConnection(#[from] CreateConnectionError),
    #[error(transparent)]
    Updateonnection(#[from] UpdateConnectionError),
    #[error(transparent)]
    DeleteConnection(#[from] DeleteConnectionError),

    #[error(transparent)]
    Storage(#[from] StorageError),

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}
