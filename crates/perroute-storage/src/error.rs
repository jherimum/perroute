#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Query error: {0}")]
    Query(sqlx::Error),

    #[error("Connection error: {0}")]
    Connection(sqlx::Error),

    #[error("Migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("Transaction error: {0}")]
    Tx(sqlx::Error),
}

impl From<sqlx::Error> for StorageError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => StorageError::Query(e),
            _ => StorageError::Query(e),
        }
    }
}
