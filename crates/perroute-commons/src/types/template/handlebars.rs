use std::borrow::Cow;

use super::{TemplateError, TemplateRender, TemplateValidator};
use handlebars::Template;
use serde::Serialize;
use validator::ValidationError;

#[derive(Debug)]
pub struct Handlebars<'r> {
    handlebars: handlebars::Handlebars<'r>,
}

impl<'r> Handlebars<'r> {
    pub fn new() -> Self {
        let mut handlebars = handlebars::Handlebars::new();
        handlebars.set_strict_mode(true);
        Self { handlebars }
    }
}

impl<'r> Default for Handlebars<'r> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'r, D: Serialize> TemplateRender<D> for Handlebars<'r> {
    fn render(&self, template: &str, data: &D) -> Result<String, TemplateError> {
        self.handlebars
            .render_template(template, data)
            .map_err(|e| TemplateError::RenderError(e.to_string()))
    }
}

impl<'r> TemplateValidator for Handlebars<'r> {
    fn validate(template: &str) -> Result<(), ValidationError> {
        match Template::compile(template) {
            Ok(_) => Ok(()),
            Err(_) => Err(ValidationError {
                code: Cow::Borrowed("template"),
                message: Some(Cow::Borrowed("Invalid template")),
                params: Default::default(),
            }),
        }
    }
}
