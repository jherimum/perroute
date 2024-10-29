use perroute_storage::repository;

#[derive(Debug, thiserror::Error)]
pub enum CommandBusError {
    #[error("Command handler not found for command: {0}")]
    CommandHandlerNotFound(String),

    #[error("Repository error: {0}")]
    Repository(#[from] repository::Error),
}
