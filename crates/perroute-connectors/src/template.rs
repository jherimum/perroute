use perroute_commons::types::template::{TemplateData, TemplateError};

pub trait DispatchTemplate: Send + Sync {
    fn render_text(&self, data: &TemplateData) -> Result<Option<String>, TemplateError>;
    fn render_html(&self, data: &TemplateData) -> Result<Option<String>, TemplateError>;
}
