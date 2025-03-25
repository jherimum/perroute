use std::future::Future;
use sqlx::{
    postgres::{PgArguments, PgRow},
    query::{Query, QueryAs},
    FromRow, Postgres, QueryBuilder,
};
use datasource::Connection;

pub mod business_unit;
pub mod channel;
pub mod datasource;
pub mod dispatcher_log;
pub mod event;
pub mod message;
pub mod message_type;
pub mod route;
pub mod template_assignment;

macro_rules! execute_ {
    ($source:expr,$query:expr) => {
        match $source {
            Connection::Pool(pool) => $query.execute(pool).await,
            Connection::Tx(tx) => {
                let mut x = tx.lock().await;
                $query.execute(x.as_mut()).await
            }
        }
    };
}

macro_rules! fetch_all_ {
    ($source:expr,$query:expr) => {
        match $source {
            Connection::Pool(pool) => $query.fetch_all(pool).await,
            Connection::Tx(tx) => {
                let mut x = tx.lock().await;
                $query.fetch_all(x.as_mut()).await
            }
        }
    };
}

macro_rules! fetch_optional_ {
    ($source:expr,$query:expr) => {
        match $source {
            Connection::Pool(pool) => $query.fetch_optional(pool).await,
            Connection::Tx(tx) => {
                let mut x = tx.lock().await;
                $query.fetch_optional(x.as_mut()).await
            }
        }
    };
}

macro_rules! fetch_one_ {
    ($source:expr,$query:expr) => {
        match $source {
            Connection::Pool(pool) => $query.fetch_one(pool).await,
            Connection::Tx(tx) => {
                let mut x = tx.lock().await;
                $query.fetch_one(x.as_mut()).await
            }
        }
    };
}

pub type ActiveRecordResult<T> = Result<T, ActiveRecordError>;

#[derive(Debug, thiserror::Error)]
pub enum ActiveRecordError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Connection invalid state")]
    ConnectionInvalidState,
}

pub trait Model: Unpin + Sync + Send + for<'r> FromRow<'r, PgRow> {
    type Create;

    fn update_query(&self) -> QueryAs<'_, Postgres, Self, PgArguments>;

    fn create_query<'q>(
        create: Self::Create,
    ) -> QueryAs<'q, Postgres, Self, PgArguments>;

    fn destroy_query(&self) -> Query<'_, Postgres, PgArguments>;
}

pub enum Projection {
    Row,
    Count,
    Field(String),
    Delete,
}

impl Projection {
    pub fn query_builder<'qb>(
        &self,
        prefix: Option<&'static str>,
    ) -> QueryBuilder<'qb, Postgres> {
        let prefix = match prefix {
            Some(p) => format!("{}.", p),
            None => "".to_owned(),
        };
        QueryBuilder::new(match self {
            Self::Row => format!("SELECT {}*", prefix),
            Self::Count => format!("SELECT COUNT({}*)", prefix),
            Self::Field(name) => format!("SELECT {}{}", prefix, name),
            Self::Delete => format!("DELETE FROM {}*", prefix),
        })
    }
}

pub trait ModelQuery<M>
where
    M: Model,
{
    fn build(&self, projection: Projection) -> QueryBuilder<'_, Postgres>;
}

impl<C: AsRef<Connection>, M: Model> ActiveRecord<C> for M {}

pub trait ActiveRecord<C>
where
    Self: Model,
    C: AsRef<Connection>,
{
    fn query<MQ: ModelQuery<Self>>(
        source: C,
        query: MQ,
    ) -> impl Future<Output = ActiveRecordResult<Vec<Self>>> {
        async move {
            Ok(fetch_all_!(
                source.as_ref(),
                query.build(Projection::Row).build_query_as()
            )?)
        }
    }

    fn count<MQ: ModelQuery<Self>>(
        source: C,
        query: MQ,
    ) -> impl Future<Output = ActiveRecordResult<i64>> {
        async move {
            Ok(fetch_one_!(
                source.as_ref(),
                query.build(Projection::Count).build_query_scalar()
            )?)
        }
    }

    fn exists<MQ: ModelQuery<Self>>(
        source: C,
        query: MQ,
    ) -> impl Future<Output = ActiveRecordResult<bool>> {
        async move { Ok(Self::count(source, query).await? > 0) }
    }

    fn fetch_optional<MQ: ModelQuery<Self>>(
        source: C,
        query: MQ,
    ) -> impl Future<Output = ActiveRecordResult<Option<Self>>> {
        async move {
            Ok(fetch_optional_!(
                source.as_ref(),
                query.build(Projection::Row).build_query_as()
            )?)
        }
    }

    fn fetch_one<MQ: ModelQuery<Self>>(
        source: C,
        query: MQ,
    ) -> impl Future<Output = ActiveRecordResult<Self>> {
        async move {
            Ok(fetch_one_!(
                source.as_ref(),
                query.build(Projection::Row).build_query_as()
            )?)
        }
    }

    fn update(
        self,
        source: C,
    ) -> impl Future<Output = ActiveRecordResult<Self>> {
        async move { Ok(fetch_one_!(source.as_ref(), self.update_query())?) }
    }

    fn create(
        source: C,
        create: Self::Create,
    ) -> impl Future<Output = ActiveRecordResult<Self>> {
        async move {
            Ok(fetch_one_!(
                source.as_ref(),
                Self::create_query(create.into())
            )?)
        }
    }

    fn delete<MQ: ModelQuery<Self>>(
        source: C,
        query: MQ,
    ) -> impl Future<Output = ActiveRecordResult<u64>> {
        async move {
            Ok(execute_!(
                source.as_ref(),
                query.build(Projection::Row).build()
            )?
            .rows_affected())
        }
    }

    fn destroy(
        self,
        source: C,
    ) -> impl Future<Output = ActiveRecordResult<bool>> {
        // Self::on_destroy(&source, &self).await?;
        async move {
            Ok(execute_!(source.as_ref(), self.destroy_query())?
                .rows_affected()
                > 0)
        }
    }
}
