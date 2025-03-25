use std::future::Future;

use perroute_commons::types::{id::Id, ProviderId, Timestamp};

use crate::models::dispatcher_log::{DispatcherError, DispatcherLog};
use super::{datasource::Connection, ActiveRecordResult, Model};

pub trait DispatcherLogActiveRecord<C: AsRef<Connection>> {
    fn save_all(
        conn: C,
        logs: Vec<DispatcherLog>,
    ) -> impl Future<Output = ActiveRecordResult<Vec<DispatcherLog>>>;
}

impl Model for DispatcherLog {
    type Create = CreateDispatcherLog;

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

pub struct CreateDispatcherLog {
    message_id: Id,
    provider_id: ProviderId,
    error: Option<DispatcherError>,
    created_at: Timestamp,
}
