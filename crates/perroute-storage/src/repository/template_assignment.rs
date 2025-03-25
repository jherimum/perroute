use crate::models::template_assignment::TemplateAssignment;

use super::RepositoryResult;

#[async_trait::async_trait]
pub trait TemplateAssignmentRepository {
    async fn find_template_assingment_for_dispatch(
        &self,
    ) -> RepositoryResult<Option<TemplateAssignment>>;
}
