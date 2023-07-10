use perroute_commons::types::id::Id;
use sqlx::{postgres::PgRow, FromRow, PgExecutor, Postgres, QueryBuilder, Row};

#[async_trait::async_trait]
pub trait FetchableModel<Q: ModelQuery<M>, M> {
    async fn count<'e, E: PgExecutor<'e>>(exec: E, query: Q) -> Result<i64, sqlx::Error>;
    async fn query<'e, E: PgExecutor<'e>>(exec: E, query: Q) -> Result<Vec<M>, sqlx::Error>;
    async fn find<'e, E: PgExecutor<'e>>(exec: E, query: Q) -> Result<Option<M>, sqlx::Error>;
}

#[async_trait::async_trait]
impl<M, Q: ModelQuery<M> + ModelQueryFetch<M> + Send + Sync + 'static> FetchableModel<Q, M> for M {
    async fn count<'e, E: PgExecutor<'e>>(exec: E, query: Q) -> Result<i64, sqlx::Error> {
        query.count(exec).await
    }
    async fn query<'e, E: PgExecutor<'e>>(exec: E, query: Q) -> Result<Vec<M>, sqlx::Error> {
        query.many(exec).await
    }
    async fn find<'e, E: PgExecutor<'e>>(exec: E, query: Q) -> Result<Option<M>, sqlx::Error> {
        query.one(exec).await
    }
}

#[async_trait::async_trait]
pub trait ModelQueryFetch<M> {
    async fn count<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<i64, sqlx::Error>;
    async fn one<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Option<M>, sqlx::Error>;
    async fn many<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Vec<M>, sqlx::Error>;
    async fn ids<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Vec<Id>, sqlx::Error>;
}

#[async_trait::async_trait]
impl<Q, M> ModelQueryFetch<M> for Q
where
    Q: ModelQuery<M> + Sync,
    M: Unpin + Sync + Send + for<'a> FromRow<'a, PgRow>,
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

impl Projection {
    pub fn query_builder<'qb>(&self) -> QueryBuilder<'qb, Postgres> {
        QueryBuilder::new(match self {
            Projection::Row => "SELECT *",
            Projection::Count => "SELECT COUNT(*)",
            Projection::Id => "SELECT id",
        })
    }
}
