use business_unit::BusinessUnitRepository;
use dispatcher_log::DispatcherLogRepository;
use message::MessageRepository;
use message_type::MessageTypeRepository;
use route::RouteRepository;
use template_assignment::TemplateAssignmentRepository;

pub mod business_unit;
pub mod dispatcher_log;
pub mod message;
pub mod message_type;
pub mod route;
pub mod template_assignment;

#[cfg(feature = "pgrepository")]
pub mod pgrepository;

pub type RepositoryResult<T> = Result<T, RepositoryError>;

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("{0}")]
    SqlxError(#[from] sqlx::Error),
}

#[async_trait::async_trait]
pub trait Repository:
    MessageRepository
    + MessageTypeRepository
    + BusinessUnitRepository
    + RouteRepository
    + DispatcherLogRepository
    + TemplateAssignmentRepository
{
    type Tx: TransactionalRepository + Send + Sync;

    async fn begin_transaction(&self) -> RepositoryResult<Self::Tx>;
}

#[async_trait::async_trait]
pub trait TransactionalRepository:
    MessageRepository
    + MessageTypeRepository
    + BusinessUnitRepository
    + RouteRepository
    + DispatcherLogRepository
    + TemplateAssignmentRepository
{
    async fn commit(self) -> RepositoryResult<()>;
    async fn rollback(self) -> RepositoryResult<()>;
}
