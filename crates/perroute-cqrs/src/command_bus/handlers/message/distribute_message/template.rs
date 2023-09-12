use std::sync::Arc;

use perroute_commons::types::template::{TemplateData, TemplateError, TemplateRender};
use perroute_connectors::template::DispatchTemplate;
use perroute_storage::models::template::Template;

pub struct InnerDispatchTemplate {
    pub template: Arc<Template>,
    pub render: Arc<dyn TemplateRender<TemplateData>>,
}

impl DispatchTemplate for InnerDispatchTemplate {
    fn render_text(&self, data: &TemplateData) -> Result<Option<String>, TemplateError> {
        self.template
            .text()
            .as_ref()
            .map(|t| t.render(self.render.as_ref(), data))
            .transpose()
    }

    fn render_html(&self, data: &TemplateData) -> Result<Option<String>, TemplateError> {
        self.template
            .html()
            .as_ref()
            .map(|t| t.render(self.render.as_ref(), data))
            .transpose()
    }

    fn render_subject(&self, data: &TemplateData) -> Result<Option<String>, TemplateError> {
        self.template
            .subject()
            .as_ref()
            .map(|t| t.render(self.render.as_ref(), data))
            .transpose()
    }
}
