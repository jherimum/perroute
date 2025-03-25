use super::{context::TemplateRenderContext, TemplateError, TemplateRender};
use handlebars::Handlebars;
use tap::TapFallible;

pub struct HandlebarsTemplateRender<'reg> {
    handlebars: Handlebars<'reg>,
}

impl Default for HandlebarsTemplateRender<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl HandlebarsTemplateRender<'_> {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(true);
        handlebars.set_dev_mode(true);
        // handlebars.set_helpers_path("templates/helpers");
        // handlebars.set_templates_path("templates");
        // handlebars.set_partials_path("templates/partials");

        Self { handlebars }
    }
}

impl TemplateRender for HandlebarsTemplateRender<'_> {
    fn render(
        &self,
        template: &str,
        context: &TemplateRenderContext,
    ) -> Result<String, TemplateError> {
        self.handlebars
            .render_template(template, context)
            .tap_err(|e| log::error!("Error rendering template: {}", e))
            .map_err(|e| TemplateError::RenderError(e.to_string()))
    }
}
