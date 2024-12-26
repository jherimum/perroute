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
use std::{fmt::Debug, sync::Arc};
use template_assignment::TemplateAssignmentRepository;
use tokio::sync::Mutex;

use crate::database::build_pool;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("SQLx error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Storage error: {0}")]
    InvalidRepositoryState(String),
}

pub type RepositoryResult<T> = Result<T, Error>;

#[async_trait::async_trait]
pub trait TransactedRepository: Repository {
    async fn commit(self) -> RepositoryResult<()>;
    async fn rollback(self) -> RepositoryResult<()>;
}

#[async_trait::async_trait]
pub trait Repository:
    Send
    + Sync
    + BusinessUnitRepository
    + MessageTypeRepository
    + PayloadExampleRepository
    + ChannelRepository
    + RouteRepository
    + MessageRepository
    + TemplateAssignmentRepository
    + DispatcherLogRepository
    + EventRepository
{
    type TR: TransactedRepository;

    async fn begin(&self) -> RepositoryResult<Self::TR>;
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

    pub async fn from_settings(
        settings: &DatabaseSettings,
    ) -> RepositoryResult<Self> {
        let pool = build_pool(settings).await?;
        Ok(Self::new(pool))
    }
}

#[async_trait::async_trait]
impl TransactedRepository for PgRepository {
    async fn rollback(self) -> RepositoryResult<()> {
        match self.source {
            Source::Tx(tx) => {
                let tx = Arc::try_unwrap(tx).unwrap().into_inner();
                tx.rollback().await?;
                Ok(())
            }
            _ => Err(Error::InvalidRepositoryState(
                "Invalid repository state".to_owned(),
            )),
        }
    }

    async fn commit(self) -> RepositoryResult<()> {
        match self.source {
            Source::Tx(tx) => {
                let tx = Arc::try_unwrap(tx).unwrap().into_inner();
                tx.commit().await?;
                Ok(())
            }
            _ => Err(Error::InvalidRepositoryState(
                "Invalid repository state".to_owned(),
            )),
        }
    }
}

#[async_trait::async_trait]
impl Repository for PgRepository {
    type TR = PgRepository;

    async fn begin(&self) -> RepositoryResult<PgRepository> {
        match &self.source {
            Source::Pool(p) => {
                let tx = p.begin().await?;
                Ok(PgRepository {
                    source: Source::Tx(Arc::new(Mutex::new(tx))),
                })
            }
            Source::Tx(_) => Err(Error::InvalidRepositoryState(
                "A transaction is already in progress".to_owned(),
            )),
        }
    }
}
