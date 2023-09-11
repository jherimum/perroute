use perroute_commons::types::template::{TemplateData, TemplateError};
use perroute_connectors::template::DispatchTemplate;
use perroute_storage::models::template::Template;

pub struct InnerDispatchTemplate<'t>(pub &'t Template);

impl DispatchTemplate for InnerDispatchTemplate<'_> {
    fn render_text(&self, data: &TemplateData) -> Result<Option<String>, TemplateError> {
        todo!()
    }

    fn render_html(&self, data: &TemplateData) -> Result<Option<String>, TemplateError> {
        todo!()
    }

    fn render_subject(&self, data: &TemplateData) -> Result<Option<String>, TemplateError> {
        todo!()
    }
}
