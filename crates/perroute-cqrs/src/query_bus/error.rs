use perroute_storage::error::StorageError;

use super::queries::QueryType;

#[derive(thiserror::Error, Debug)]
pub enum QueryBusError {
    #[error("Handler not found for query {0}")]
    HandlerNotFound(QueryType),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),

    #[error(transparent)]
    StorageError(#[from] StorageError),

    #[error("{0}")]
    EntityNotFound(String),
}
