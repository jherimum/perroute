pub mod context;
pub mod handlebars;

use context::TemplateRenderContext;

#[derive(Debug, thiserror::Error)]
pub enum TemplateError {
    #[error("Template render error: {0}")]
    RenderError(String),
}

pub trait TemplateRender: Send + Sync {
    fn render(
        &self,
        template: &str,
        context: &TemplateRenderContext,
    ) -> Result<String, TemplateError>;
}

pub trait Renderable {
    fn render(
        &self,
        render: &dyn TemplateRender,
        context: &TemplateRenderContext,
    ) -> Result<String, TemplateError>;
}

impl<S: AsRef<str>> Renderable for S {
    fn render(
        &self,
        render: &dyn TemplateRender,
        context: &TemplateRenderContext,
    ) -> Result<String, TemplateError> {
        render.render(self.as_ref(), context)
    }
}
