use serde::Serialize;

use super::{TemplateError, TemplateRender};

#[derive(Debug)]
pub struct HandlebarsTemplateRender<'r> {
    handlebars: handlebars::Handlebars<'r>,
}

impl<'r> HandlebarsTemplateRender<'r> {
    pub fn new() -> Self {
        let mut handlebars = handlebars::Handlebars::new();
        handlebars.set_strict_mode(true);
        Self { handlebars }
    }
}

impl<'r> Default for HandlebarsTemplateRender<'r> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'r, D: Serialize> TemplateRender<D> for HandlebarsTemplateRender<'r> {
    fn render(&self, template: &str, data: &D) -> Result<String, TemplateError> {
        self.handlebars
            .render_template(template, data)
            .map_err(|e| TemplateError::RenderError(e.to_string()))
    }
}
