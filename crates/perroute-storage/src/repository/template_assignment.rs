use super::{PgRepository, RepositoryResult};
use crate::models::template_assignment::TemplateAssignment;
use perroute_commons::types::{dispatch_type::DispatchType, id::Id, Timestamp};

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

#[async_trait::async_trait]
pub trait TemplateAssignmentRepository {
    async fn query(
        &self,
        query: TemplateAssignmentQuery<'_>,
    ) -> RepositoryResult<Vec<TemplateAssignment>>;

    async fn find(
        &self,
        query: TemplateAssignmentQuery<'_>,
    ) -> RepositoryResult<Option<TemplateAssignment>>;
}

#[async_trait::async_trait]
impl TemplateAssignmentRepository for PgRepository {
    async fn query(
        &self,
        query: TemplateAssignmentQuery<'_>,
    ) -> RepositoryResult<Vec<TemplateAssignment>> {
        todo!()
    }

    async fn find(
        &self,
        query: TemplateAssignmentQuery<'_>,
    ) -> RepositoryResult<Option<TemplateAssignment>> {
        todo!()
    }
}
