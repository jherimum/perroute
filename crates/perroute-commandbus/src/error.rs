use perroute_storage::error::StorageError;

use crate::command::business_unit::create_business_unit::CreateBusinessUnitError;

#[derive(Debug, thiserror::Error)]
pub enum CommandBusError {
    #[error("")]
    ActorNotSupported,

    #[error(transparent)]
    CreateBusinessUnit(#[from] CreateBusinessUnitError),

    #[error(transparent)]
    Storage(#[from] StorageError),

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}
