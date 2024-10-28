pub mod business_units;

use business_units::BusinessUnitRepository;
use perroute_commons::configuration::settings::DatabaseSettings;
use sqlx::{PgPool, Postgres, Transaction};
use std::{future::Future, sync::Arc};
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

pub trait Repository: BusinessUnitRepository {
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
            _ => Err(Error::InvalidRepositoryState("Invalid repository state")),
        }
    }

    async fn commit(&self) -> RepositoryResult<()> {
        match self {
            Source::Tx(tx) => {
                let tx = Arc::try_unwrap(tx.clone()).unwrap().into_inner();
                tx.commit().await?;
                Ok(())
            }
            _ => Err(Error::InvalidRepositoryState("Invalid repository state")),
        }
    }

    async fn rollback(&self) -> RepositoryResult<()> {
        match self {
            Source::Tx(tx) => {
                let tx = Arc::try_unwrap(tx.clone()).unwrap().into_inner();
                tx.rollback().await?;
                Ok(())
            }
            _ => Err(Error::InvalidRepositoryState("Invalid repository state")),
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
