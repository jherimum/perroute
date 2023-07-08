use sqlx::{postgres::PgRow, FromRow, PgExecutor, Postgres, QueryBuilder, Row};

#[async_trait::async_trait]
pub trait ModelQueryFetch<M: for<'a> FromRow<'a, PgRow> + Send + Unpin>
where
    Self: ModelQuery<M>,
{
    async fn count<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<i64, sqlx::Error> {
        self.query_builder(true)
            .build()
            .fetch_one(exec)
            .await
            .map(|r| r.get(0))
    }
    async fn one<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Option<M>, sqlx::Error> {
        self.query_builder(false)
            .build_query_as::<M>()
            .fetch_optional(exec)
            .await
    }
    async fn many<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Vec<M>, sqlx::Error> {
        self.query_builder(false)
            .build_query_as::<M>()
            .fetch_all(exec)
            .await
    }
}

pub trait ModelQuery<M> {
    fn query_builder(&self, count: bool) -> QueryBuilder<'_, Postgres>;
}
