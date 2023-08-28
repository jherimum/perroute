use crate::{error::StorageError, DatabaseModel};
use perroute_commons::types::id::Id;
use sqlx::{postgres::PgRow, FromRow, PgExecutor, Postgres, QueryBuilder, Row};

#[async_trait::async_trait]
pub trait FetchableModel<Q: ModelQueryBuilder<M>, M> {
    async fn count<'e, E: PgExecutor<'e>>(exec: E, query: Q) -> Result<i64, StorageError>;
    async fn query<'e, E: PgExecutor<'e>>(exec: E, query: Q) -> Result<Vec<M>, StorageError>;
    async fn find<'e, E: PgExecutor<'e>>(exec: E, query: Q) -> Result<Option<M>, StorageError>;
    async fn find_one<'e, E: PgExecutor<'e>>(exec: E, query: Q) -> Result<M, StorageError>;
    async fn exists<'e, E: PgExecutor<'e>>(exec: E, query: Q) -> Result<bool, StorageError>;
    async fn ids<'e, E: PgExecutor<'e>>(exec: E, query: Q) -> Result<Vec<Id>, StorageError>;
}

#[async_trait::async_trait]
impl<Q, M> FetchableModel<Q, M> for M
where
    Q: ModelQueryBuilder<M> + Send + Sync + 'static,
    M: DatabaseModel + Unpin + Sync + Send + for<'a> FromRow<'a, PgRow>,
{
    async fn count<'e, E: PgExecutor<'e>>(exec: E, query: Q) -> Result<i64, StorageError> {
        Ok(query
            .build(Projection::Count)
            .build()
            .fetch_one(exec)
            .await
            .map(|r| r.get(0))?)
    }
    async fn query<'e, E: PgExecutor<'e>>(exec: E, query: Q) -> Result<Vec<M>, StorageError> {
        Ok(query
            .build(Projection::Row)
            .build_query_as::<M>()
            .fetch_all(exec)
            .await?)
    }

    async fn ids<'e, E: PgExecutor<'e>>(exec: E, query: Q) -> Result<Vec<Id>, StorageError> {
        Ok(query
            .build(Projection::Id)
            .build()
            .fetch_all(exec)
            .await?
            .iter()
            .map(|r| r.get(0))
            .collect::<Vec<Id>>())
    }

    async fn find<'e, E: PgExecutor<'e>>(exec: E, query: Q) -> Result<Option<M>, StorageError> {
        Ok(query
            .build(Projection::Row)
            .build_query_as::<M>()
            .fetch_optional(exec)
            .await?)
    }

    async fn find_one<'e, E: PgExecutor<'e>>(exec: E, query: Q) -> Result<M, StorageError> {
        Ok(query
            .build(Projection::Row)
            .build_query_as::<M>()
            .fetch_one(exec)
            .await?)
    }

    async fn exists<'e, E: PgExecutor<'e>>(exec: E, query: Q) -> Result<bool, StorageError> {
        Ok(query
            .build(Projection::Count)
            .build()
            .fetch_one(exec)
            .await
            .map::<i64, _>(|r| r.get(0))
            .map(|c| c > 0)?)
    }
}

pub trait ModelQueryBuilder<M> {
    fn build(&self, projection: Projection) -> QueryBuilder<'_, Postgres>;
}

pub enum Projection {
    Row,
    Count,
    Id,
}

impl Projection {
    pub fn query_builder<'qb>(&self) -> QueryBuilder<'qb, Postgres> {
        QueryBuilder::new(match self {
            Self::Row => "SELECT *",
            Self::Count => "SELECT COUNT(*)",
            Self::Id => "SELECT id",
        })
    }
}
