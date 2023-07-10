use perroute_commons::prelude::Id;
use sqlx::{postgres::PgRow, FromRow, PgExecutor, Postgres, QueryBuilder, Row};

#[async_trait::async_trait]
pub trait ModelQueryFetch<M> {
    async fn count<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<i64, sqlx::Error>;
    async fn one<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Option<M>, sqlx::Error>;
    async fn many<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Vec<M>, sqlx::Error>;
    async fn ids<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Vec<Id>, sqlx::Error>;
}

#[async_trait::async_trait]
impl<Q: ModelQuery<M> + Sync, M: Unpin + Sync + Send + for<'a> FromRow<'a, PgRow>>
    ModelQueryFetch<M> for Q
{
    async fn count<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<i64, sqlx::Error> {
        self.query_builder(Projection::Count)
            .build()
            .fetch_one(exec)
            .await
            .map(|r| r.get(0))
    }
    async fn one<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Option<M>, sqlx::Error> {
        self.query_builder(Projection::Row)
            .build_query_as::<M>()
            .fetch_optional(exec)
            .await
    }
    async fn many<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Vec<M>, sqlx::Error> {
        self.query_builder(Projection::Row)
            .build_query_as::<M>()
            .fetch_all(exec)
            .await
    }

    async fn ids<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Vec<Id>, sqlx::Error> {
        self.query_builder(Projection::Row)
            .build()
            .fetch_all(exec)
            .await
            .map(|r| r.iter().map(|r| r.get::<Id, usize>(0)).collect::<Vec<_>>())
    }
}

pub trait ModelQuery<M> {
    fn query_builder(&self, projection: Projection) -> QueryBuilder<'_, Postgres>;
}

pub enum Projection {
    Row,
    Count,
    Id,
}
