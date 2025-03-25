use std::future::Future;

use perroute_commons::types::id::Id;

use crate::models::{
    business_unit::BusinessUnit, message::Message, message_type::MessageType,
};
use super::{datasource::Connection, ActiveRecordResult, Model, ModelQuery};

pub enum MessageQuery<'q> {
    ById(&'q Id),
}

impl ModelQuery<Message> for MessageQuery<'_> {
    fn build(
        &self,
        projection: super::Projection,
    ) -> sqlx::QueryBuilder<'_, sqlx::Postgres> {
        todo!()
    }
}

impl Model for Message {
    type Create = CreateMessage;

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

pub trait MessageActiveRecord {
    fn business_unit<C: AsRef<Connection>>(
        &self,
        conn: C,
    ) -> impl Future<Output = ActiveRecordResult<BusinessUnit>>;

    fn message_type<C: AsRef<Connection>>(
        &self,
        conn: C,
    ) -> impl Future<Output = ActiveRecordResult<MessageType>>;
}

impl MessageActiveRecord for Message {
    async fn business_unit<C: AsRef<Connection>>(
        &self,
        conn: C,
    ) -> ActiveRecordResult<BusinessUnit> {
        todo!()
    }

    async fn message_type<C: AsRef<Connection>>(
        &self,
        conn: C,
    ) -> ActiveRecordResult<MessageType> {
        todo!()
    }
}

pub struct CreateMessage {}
