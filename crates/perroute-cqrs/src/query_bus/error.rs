use super::queries::QueryType;

#[derive(thiserror::Error, Debug)]
pub enum QueryBusError {
    #[error("Handler not found for query {0}")]
    HandlerNotFound(QueryType),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),

    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
}
