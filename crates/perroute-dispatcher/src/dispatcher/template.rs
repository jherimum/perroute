use std::sync::Arc;

use perroute_commons::{template::TemplateRender, types::template::Template};
use perroute_storage::{models::message::Message, repository::Repository};

use super::DigesterResult;

pub(crate) struct TemplateResolver<R, TR> {
    pub repository: Arc<R>,
    pub template_render: Arc<TR>,
}

impl<R: Repository, TR: TemplateRender> TemplateResolver<R, TR> {
    pub async fn resolve(&self, message: &Message) -> DigesterResult<Template> {
        todo!()
    }
}

/*

async fn render_template(&self, message: &Message) -> DigesterResult<Template> {
        let template_assignment = self
            .template_assignment(&message)
            .await?
            .ok_or(DigesterError::NoTemplateAssignmentElegible)?;
        let template = self.template(&message, &template_assignment).await?;
        let ctx = self.context(&message, &template_assignment).await?;
        template
            .render(self.template_render.as_ref(), &ctx)
            .map_err(DigesterError::from)
    }

    async fn template_assignment(
        &self,
        message: &Message,
    ) -> DigesterResult<Option<TemplateAssignment>> {
        let query = QueryForDispatch::builder()
            .message_type_id(message.message_type_id())
            .business_unit_id(message.business_unit_id())
            .dispatch_type(message.dispatch_type())
            .reference_date(
                message
                    .scheduled_at()
                    .as_ref()
                    .unwrap_or(message.created_at()),
            )
            .build();

        Ok(TemplateAssignmentRepository::find(self.repository.as_ref(), query.into()).await?)
    }

    async fn template(
        &self,
        message: &Message,
        template_assignment: &TemplateAssignment,
    ) -> DigesterResult<Template> {
        todo!()
    }

    async fn context(
        &self,
        message: &Message,
        template_assignment: &TemplateAssignment,
    ) -> DigesterResult<TemplateRenderContext> {
        todo!()
    }

*/
