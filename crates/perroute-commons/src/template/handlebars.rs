use super::{context::TemplateRenderContext, TemplateError, TemplateRender};
use handlebars::Handlebars;
use tap::TapFallible;

pub struct HandlebarsTemplateRender<'reg> {
    handlebars: Handlebars<'reg>,
}

impl<'reg> TemplateRender for HandlebarsTemplateRender<'reg> {
    fn render(
        &self,
        template: &str,
        context: &TemplateRenderContext,
    ) -> Result<String, TemplateError> {
        self.handlebars
            .render_template(template, context)
            .tap_err(|e| log::error!("Error rendering template: {}", e))
            .map_err(|e| TemplateError::RenderError(Box::new(e)))
    }
}