use std::future::Future;
use perroute_commons::types::{id::Id, Timestamp};
use crate::models::event::DbEvent;
use super::{
    datasource::{Connection},
    ActiveRecordResult, Model,
};

impl Model for DbEvent {
    type Create = CreateEvent;

    fn update_query(
        &self,
    ) -> sqlx::query::QueryAs<
        '_,
        sqlx::Postgres,
        Self,
        sqlx::postgres::PgArguments,
    > {
        todo!()
    }

    fn create_query<'q>(
        create: Self::Create,
    ) -> sqlx::query::QueryAs<
        'q,
        sqlx::Postgres,
        Self,
        sqlx::postgres::PgArguments,
    > {
        todo!()
    }

    fn destroy_query(
        &self,
    ) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>
    {
        todo!()
    }
}

pub struct CreateEvent {}

pub trait EventActiveRecord<C>
where
    C: AsRef<Connection>,
{
    fn set_consumed(
        conn: C,
        events: Vec<Id>,
        skipped: bool,
        timestamp: &Timestamp,
    ) -> impl Future<Output = ActiveRecordResult<()>>;

    fn unconsumed(
        conn: C,
        size: usize,
    ) -> impl Future<Output = ActiveRecordResult<Vec<DbEvent>>>;
}

impl<C: AsRef<Connection>> EventActiveRecord<C> for DbEvent {
    async fn set_consumed(
        conn: C,
        events: Vec<Id>,
        skipped: bool,
        timestamp: &Timestamp,
    ) -> ActiveRecordResult<()> {
        todo!()
    }

    async fn unconsumed(
        conn: C,
        size: usize,
    ) -> ActiveRecordResult<Vec<DbEvent>> {
        todo!()
    }
}
