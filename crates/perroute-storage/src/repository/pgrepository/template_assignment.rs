use crate::{
    models::template_assignment::TemplateAssignment,
    repository::{
        template_assignment::TemplateAssignmentRepository, RepositoryResult,
    },
};

use super::PgRepository;

#[async_trait::async_trait]
impl TemplateAssignmentRepository for PgRepository {
    async fn find_template_assingment_for_dispatch(
        &self,
    ) -> RepositoryResult<Option<TemplateAssignment>> {
        todo!()
    }
}
