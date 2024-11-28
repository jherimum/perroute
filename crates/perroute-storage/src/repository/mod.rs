pub mod business_units;
pub mod channels;
pub mod command_audit;
pub mod dispatcher_log;
pub mod events;
pub mod message;
pub mod message_types;
pub mod routes;
pub mod template_assignment;

use business_units::BusinessUnitRepository;
use channels::ChannelRepository;
use command_audit::CommandAuditRepository;
use dispatcher_log::DispatcherLogRepository;
use events::EventRepository;
use message::MessageRepository;
use message_types::{MessageTypeRepository, PayloadExampleRepository};
use perroute_commons::configuration::settings::DatabaseSettings;
use routes::RouteRepository;
use sqlx::{PgPool, Postgres, Transaction};
use std::{future::Future, sync::Arc};
use template_assignment::TemplateAssignmentRepository;
use tokio::sync::Mutex;

use crate::database::build_pool;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("SQLx error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Storage error: {0}")]
    InvalidRepositoryState(&'static str),
}

pub type RepositoryResult<T> = Result<T, Error>;

pub trait TransactedRepository: Repository {
    fn commit(self) -> impl Future<Output = RepositoryResult<()>>;
    fn rollback(self) -> impl Future<Output = RepositoryResult<()>>;
}

pub trait Repository:
    BusinessUnitRepository
    + MessageTypeRepository
    + PayloadExampleRepository
    + ChannelRepository
    + EventRepository
    + RouteRepository
    + EventRepository
    + CommandAuditRepository
    + MessageRepository
    + TemplateAssignmentRepository
    + DispatcherLogRepository
{
    fn begin(&self) -> impl Future<Output = RepositoryResult<impl TransactedRepository + Clone>>;
}

#[derive(Clone)]
pub enum Source {
    Pool(PgPool),
    Tx(Arc<Mutex<Transaction<'static, Postgres>>>),
}

impl Source {
    async fn begin(&self) -> RepositoryResult<Self> {
        match &self {
            Source::Pool(pool) => {
                let tx = pool.begin().await?;
                Ok(Source::Tx(Arc::new(Mutex::new(tx))))
            }
            _ => Err(Error::InvalidRepositoryState(
                "A transaction is already in progress",
            )),
        }
    }

    async fn commit(self) -> RepositoryResult<()> {
        match self {
            Source::Tx(tx) => match Arc::try_unwrap(tx) {
                Ok(tx) => {
                    let tx = tx.into_inner();
                    tx.commit().await?;
                    Ok(())
                }
                Err(_) => Err(Error::InvalidRepositoryState(
                    "Unexpected error when unwrapping transaction",
                )),
            },
            _ => Err(Error::InvalidRepositoryState(
                "There is no transaction to commit",
            )),
        }
    }

    async fn rollback(self) -> RepositoryResult<()> {
        match self {
            Source::Tx(tx) => match Arc::try_unwrap(tx) {
                Ok(tx) => {
                    let tx = tx.into_inner();
                    tx.rollback().await?;
                    Ok(())
                }
                Err(_) => Err(Error::InvalidRepositoryState(
                    "Unexpected error when unwrapping transaction",
                )),
            },
            _ => Err(Error::InvalidRepositoryState(
                "There is no transaction to commit",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PgRepository {
    source: Source,
}

impl PgRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            source: Source::Pool(pool),
        }
    }

    pub async fn from_settings(settings: &DatabaseSettings) -> RepositoryResult<Self> {
        let pool = build_pool(settings).await?;
        Ok(Self::new(pool))
    }
}

impl TransactedRepository for PgRepository {
    async fn rollback(self) -> RepositoryResult<()> {
        self.source.rollback().await
    }

    async fn commit(self) -> RepositoryResult<()> {
        self.source.commit().await
    }
}

impl Repository for PgRepository {
    async fn begin(&self) -> RepositoryResult<impl TransactedRepository + Clone> {
        let source = self.source.begin().await?;
        Ok(PgRepository { source })
    }
}
