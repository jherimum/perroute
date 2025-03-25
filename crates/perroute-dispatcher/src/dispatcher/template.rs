use perroute_commons::types::vars::Vars;
use perroute_storage::{
    active_record::{
        business_unit::BusinessUnitQuery, message_type::MessageTypeQuery,
    },
    models::{message::Message, template_assignment::TemplateAssignment},
    repository::{
        business_unit::BusinessUnitRepository,
        message_type::MessageTypeRepository,
        template_assignment::TemplateAssignmentRepository, RepositoryError,
    },
};
use perroute_template::{
    render::{TemplateRenderContext, TemplateRenderPlugin},
    repository::{TemplateId, TemplateLookup},
    template::{RenderedTemplateState, Template},
};
use super::DispatchError;

#[derive(Clone)]
pub struct TemplateGenerator<REPO, TRP, TR> {
    template_render_plugin: TRP,
    template_repository: TR,
    repository: REPO,
}

impl<REPO, TRP, TR> TemplateGenerator<REPO, TRP, TR>
where
    TRP: TemplateRenderPlugin,
    TR: TemplateLookup,
    REPO: BusinessUnitRepository
        + MessageTypeRepository
        + TemplateAssignmentRepository,
{
    pub fn new(
        template_render_plugin: TRP,
        template_repository: TR,
        repository: REPO,
    ) -> Self {
        Self {
            template_render_plugin,
            template_repository,
            repository,
        }
    }

    async fn fetch_template_assignment(
        &self,
        _: &Message,
    ) -> Result<Option<TemplateAssignment>, RepositoryError> {
        TemplateAssignmentRepository::find_template_assingment_for_dispatch(
            &self.repository,
        )
        .await
    }

    async fn build_vars(
        &self,
        template_assignment: &TemplateAssignment,
        message: &Message,
    ) -> Result<Vars, DispatchError> {
        let bu = BusinessUnitRepository::get_bu(
            &self.repository,
            BusinessUnitQuery::ById(message.business_unit_id()),
        )
        .await?;

        let message_type = MessageTypeRepository::get_message_type(
            &self.repository,
            MessageTypeQuery::ById(message.business_unit_id()),
        )
        .await?;

        Ok(bu
            .vars()
            .merge(message_type.vars())
            .merge(template_assignment.vars()))
    }

    pub async fn generate(
        &self,
        message: &Message,
    ) -> Result<Template<RenderedTemplateState>, DispatchError> {
        let template_assignment = match self
            .fetch_template_assignment(message)
            .await?
        {
            Some(template_assignment) => template_assignment,
            None => return Err(DispatchError::NoTemplateAssignmentEligible),
        };
        let template_id =
            match template_assignment.template_id(message.dispatch_type()) {
                Some(id) => id,
                None => {
                    return Err(DispatchError::UnexpectedError(
                        "the respective id was not found".to_owned().into(),
                    ))
                }
            };

        let template_path = TemplateId::new(
            template_assignment.id(),
            message.dispatch_type(),
            &template_id,
        );

        let template =
            match self.template_repository.get(&template_path).await? {
                Some(template) => template,
                None => {
                    return Err(DispatchError::TemplateNotFound("".to_owned()))
                }
            };

        let vars = self.build_vars(&template_assignment, &message).await?;
        let ctx = TemplateRenderContext::new(message.payload(), &vars);
        let renderer = self.template_render_plugin.renderer(ctx);

        Ok(template.render(renderer.as_ref())?)
    }
}
