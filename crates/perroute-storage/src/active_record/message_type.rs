use perroute_commons::types::{code::Code, id::Id};

use crate::models::message_type::MessageType;

use super::{Model, ModelQuery};

pub enum MessageTypeQuery<'q> {
    ById(&'q Id),
    ByCode(&'q Code),
    All,
}

impl ModelQuery<MessageType> for MessageTypeQuery<'_> {
    fn build(
        &self,
        projection: super::Projection,
    ) -> sqlx::QueryBuilder<'_, sqlx::Postgres> {
        todo!()
    }
}

impl Model for MessageType {
    type Create = CreateMessageType;

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

pub struct CreateMessageType {}
