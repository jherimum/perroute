use std::future::Future;

use perroute_commons::types::{dispatch_type::DispatchType, id::Id};

use crate::models::{channel::Channel, message_type::MessageType, route::Route};

use super::{
    business_unit, datasource::Connection, ActiveRecordResult, Model,
    ModelQuery,
};

pub enum RouteQuery<'q> {
    ById(&'q Id),
    ActiveByBusinessUnitAndDispatchType(
        &'q ActiveByBusinessUnitAndDispatchTypeQuery<'q>,
    ),
}

pub struct ActiveByBusinessUnitAndDispatchTypeQuery<'q> {
    pub business_unit_id: &'q Id,
    pub message_type_id: &'q Id,
    pub dispatch_type: &'q DispatchType,
}

impl ModelQuery<Route> for RouteQuery<'_> {
    fn build(
        &self,
        projection: super::Projection,
    ) -> sqlx::QueryBuilder<'_, sqlx::Postgres> {
        todo!()
    }
}

impl Model for Route {
    type Create = CreateRoute;

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

pub struct CreateRoute {}

pub trait RouteActiveRecord {
    fn routes_to_dispatch<C: AsRef<Connection>>(
        conn: C,
        business_unit_id: &Id,
        message_type_id: &Id,
        dispatch_type: &DispatchType,
    ) -> impl Future<Output = ActiveRecordResult<Vec<Route>>>;

    fn channel<C: AsRef<Connection>>(
        &self,
        conn: C,
    ) -> impl Future<Output = ActiveRecordResult<Channel>>;

    fn message_type<C: AsRef<Connection>>(
        &self,
        conn: C,
    ) -> impl Future<Output = ActiveRecordResult<MessageType>>;
}

impl RouteActiveRecord for Route {
    async fn routes_to_dispatch<C: AsRef<Connection>>(
        conn: C,
        business_unit_id: &Id,
        message_type_id: &Id,
        dispatch_type: &DispatchType,
    ) -> ActiveRecordResult<Vec<Route>> {
        todo!()
    }

    async fn channel<C: AsRef<Connection>>(
        &self,
        conn: C,
    ) -> ActiveRecordResult<Channel> {
        todo!()
    }

    async fn message_type<C: AsRef<Connection>>(
        &self,
        conn: C,
    ) -> ActiveRecordResult<MessageType> {
        todo!()
    }
}
