use perroute_commons::{
    template::{context::TemplateRenderContext, TemplateRender},
    types::{dispatch_type::DispatchType, id::Id, template::Template, vars::Vars},
};
use perroute_storage::{
    models::{message::Message, template_assignment::TemplateAssignment},
    repository::{
        template_assignment::{QueryForDispatch, TemplateAssignmentRepository},
        Repository,
    },
};
use std::sync::Arc;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    RepositoryError(#[from] perroute_storage::repository::Error),

    #[error("{0}")]
    TemplateError(#[from] perroute_commons::template::TemplateError),
}

pub(crate) struct TemplateResolver<R, TR> {
    pub repository: Arc<R>,
    pub template_render: Arc<TR>,
}

impl<R: Repository, TR> AsRef<R> for TemplateResolver<R, TR> {
    fn as_ref(&self) -> &R {
        &*self.repository
    }
}

impl<R: Repository, TR: TemplateRender> TemplateResolver<R, TR> {
    async fn template_assignment(&self, message: &Message) -> Result<Option<TemplateAssignment>> {
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

        Ok(TemplateAssignmentRepository::find(&*self.repository, query.into()).await?)
    }

    async fn retrieve_template(&self, id: &Id) -> Result<Option<Template>> {
        todo!()
    }

    async fn raw_template(
        &self,
        message: &Message,
        template_assignment: &TemplateAssignment,
    ) -> Result<Option<Template>> {
        if let Some(id) = match message.dispatch_type() {
            DispatchType::Sms => template_assignment.sms_template_id(),
            DispatchType::Email => template_assignment.email_template_id(),
            DispatchType::Push => template_assignment.push_template_id(),
        } {
            return Ok(self.retrieve_template(&id).await?);
        }

        Ok(None)
    }

    async fn vars<'ctx>(
        &self,
        message: &'ctx Message,
        template_assignment: &TemplateAssignment,
    ) -> Result<Vars> {
        // let bu = BusinessUnitRepository::get(self.as_ref(), &message.business_unit_id()).await?;
        // let message_type =
        //     MessageTypeRepository::get(self.as_ref(), message.message_type_id()).await?;

        // Ok(bu
        //     .vars()
        //     .merge(message_type.vars())
        //     .merge(template_assignment.vars()))
        todo!()
    }

    pub async fn resolve(&self, message: &Message) -> Result<Option<Template>> {
        let template_assignment = match self.template_assignment(message).await? {
            Some(template) => template,
            None => return Ok(None),
        };

        let raw_template = match self.raw_template(message, &template_assignment).await? {
            Some(template) => template,
            None => return Ok(None),
        };

        let vars = self.vars(message, &template_assignment).await?;

        Ok(Some(raw_template.render(
            &*self.template_render,
            &TemplateRenderContext::new(message.payload(), &vars),
        )?))
    }
}
