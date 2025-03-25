use perroute_commons::types::{dispatch_type::DispatchType, id::Id, Timestamp};
use crate::models::template_assignment::TemplateAssignment;
use super::{Model, ModelQuery};

pub enum TemplateAssignmentQuery<'q> {
    ForDispatch(QueryForDispatch<'q>),
}

#[derive(Debug, bon::Builder)]
pub struct QueryForDispatch<'q> {
    business_unit_id: &'q Id,
    message_type_id: &'q Id,
    dispatch_type: &'q DispatchType,
    reference_date: &'q Timestamp,
}

impl<'q> From<QueryForDispatch<'q>> for TemplateAssignmentQuery<'q> {
    fn from(query: QueryForDispatch<'q>) -> Self {
        TemplateAssignmentQuery::ForDispatch(query)
    }
}

impl ModelQuery<TemplateAssignment> for TemplateAssignmentQuery<'_> {
    fn build(
        &self,
        projection: super::Projection,
    ) -> sqlx::QueryBuilder<'_, sqlx::Postgres> {
        todo!()
    }
}

impl Model for TemplateAssignment {
    type Create = CreateTemplateAssignment;

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

pub struct CreateTemplateAssignment {}
