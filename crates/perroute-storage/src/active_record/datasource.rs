use std::sync::Arc;
use sqlx::{PgPool, Postgres, Transaction};
use tokio::sync::Mutex;
use super::{ActiveRecordError, ActiveRecordResult};

#[derive(Clone, Debug)]
pub enum Connection {
    Pool(PgPool),
    Tx(Arc<Mutex<Transaction<'static, Postgres>>>),
}

#[derive(Debug, Clone)]
pub struct NonTransactionalDataSource;

#[derive(Debug)]
pub struct TransactionalDataSource;

#[derive(Debug, Clone)]
pub struct DataSource<S> {
    source: Connection,
    phantom: std::marker::PhantomData<S>,
}

impl<S> AsRef<Connection> for DataSource<S> {
    fn as_ref(&self) -> &Connection {
        &self.source
    }
}

impl DataSource<NonTransactionalDataSource> {
    pub fn new(pool: PgPool) -> Self {
        Self {
            source: Connection::Pool(pool),
            phantom: std::marker::PhantomData,
        }
    }
}

impl DataSource<NonTransactionalDataSource> {
    pub async fn begin_transaction(
        &self,
    ) -> ActiveRecordResult<DataSource<TransactionalDataSource>> {
        match &self.source {
            Connection::Pool(pool) => Ok(DataSource {
                source: Connection::Tx(Arc::new(Mutex::new(
                    pool.begin().await?,
                ))),
                phantom: std::marker::PhantomData,
            }),
            Connection::Tx(_) => Err(ActiveRecordError::ConnectionInvalidState),
        }
    }
}

impl DataSource<TransactionalDataSource> {
    pub async fn commit(self) -> ActiveRecordResult<()> {
        match self.source {
            Connection::Pool(_) => {
                Err(ActiveRecordError::ConnectionInvalidState)
            }
            Connection::Tx(tx) => {
                let tx = Arc::try_unwrap(tx).unwrap().into_inner();
                tx.commit().await?;
                Ok(())
            }
        }
    }

    pub async fn rollback(self) -> ActiveRecordResult<()> {
        match self.source {
            Connection::Pool(_) => {
                Err(ActiveRecordError::ConnectionInvalidState)
            }
            Connection::Tx(tx) => {
                let tx = Arc::try_unwrap(tx).unwrap().into_inner();
                tx.rollback().await?;
                Ok(())
            }
        }
    }
}
