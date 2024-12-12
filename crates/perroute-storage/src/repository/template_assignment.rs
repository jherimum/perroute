use std::future::Future;

use perroute_commons::types::{dispatch_type::DispatchType, id::Id, Timestamp};

use crate::models::template_assignment::TemplateAssignment;

use super::{PgRepository, RepositoryResult};

pub enum TemplateAssignmentQuery<'q> {
    ForDispatch(QueryForDispatch<'q>),
}

pub struct QueryForDispatch<'q> {
    pub business_unit_id: &'q Id,
    pub message_type_id: &'q Id,
    pub dispatch_type: &'q DispatchType,
    pub date_reference: &'q Timestamp,
}

#[async_trait::async_trait]
pub trait TemplateAssignmentRepository {
    async fn query(
        &self,
        query: &TemplateAssignmentQuery<'_>,
    ) -> RepositoryResult<Vec<TemplateAssignment>>;
}

#[async_trait::async_trait]
impl TemplateAssignmentRepository for PgRepository {
    async fn query(
        &self,
        query: &TemplateAssignmentQuery<'_>,
    ) -> RepositoryResult<Vec<TemplateAssignment>> {
        todo!()
    }
}
