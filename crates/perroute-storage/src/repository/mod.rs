pub mod business_units;
pub mod channels;
pub mod dispatcher_log;
pub mod events;
pub mod message;
pub mod message_types;
pub mod routes;
pub mod template_assignment;

use business_units::BusinessUnitRepository;
use channels::ChannelRepository;
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
    + RouteRepository
    + MessageRepository
    + TemplateAssignmentRepository
    + DispatcherLogRepository
    + EventRepository
{
    fn begin(&self) -> impl Future<Output = RepositoryResult<impl TransactedRepository + Clone>>;
}

#[derive(Clone, Debug)]
pub enum Source {
    Pool(PgPool),
    Tx(Arc<Mutex<Transaction<'static, Postgres>>>),
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
        match self.source {
            Source::Tx(tx) => {
                let tx = Arc::try_unwrap(tx).unwrap().into_inner();
                tx.rollback().await?;
                Ok(())
            }
            _ => Err(Error::InvalidRepositoryState("Invalid repository state")),
        }
    }

    async fn commit(self) -> RepositoryResult<()> {
        match self.source {
            Source::Tx(tx) => {
                let tx = Arc::try_unwrap(tx).unwrap().into_inner();
                tx.commit().await?;
                Ok(())
            }
            _ => Err(Error::InvalidRepositoryState("Invalid repository state")),
        }
    }
}

impl Repository for PgRepository {
    async fn begin(&self) -> RepositoryResult<impl TransactedRepository + Clone> {
        match &self.source {
            Source::Pool(p) => {
                let tx = p.begin().await?;
                Ok(PgRepository {
                    source: Source::Tx(Arc::new(Mutex::new(tx))),
                })
            }
            Source::Tx(_) => Err(Error::InvalidRepositoryState(
                "A transaction is already in progress",
            )),
        }
    }
}
