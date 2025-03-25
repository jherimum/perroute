#[cfg(feature = "handlebars")]
pub mod handlebars;

use perroute_commons::types::{vars::Vars, Payload};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[cfg(feature = "handlebars")]
    #[error("")]
    HandlebarsError(#[from] handlebars::Error),

    #[cfg(test)]
    #[error("{0}")]
    FailedTemplateRenderPluginError(String),
}

#[derive(Debug, Serialize)]
pub struct TemplateRenderContext<'ctx> {
    payload: &'ctx Payload,
    vars: &'ctx Vars,
}

impl<'ctx> TemplateRenderContext<'ctx> {
    pub fn new(payload: &'ctx Payload, vars: &'ctx Vars) -> Self {
        Self { payload, vars }
    }

    pub fn payload(&self) -> &Payload {
        &self.payload
    }

    pub fn vars(&self) -> &Vars {
        &self.vars
    }
}

pub trait TemplateRenderPlugin {
    fn renderer<'c>(
        &'c self,
        context: TemplateRenderContext<'c>,
    ) -> Box<dyn Renderer + 'c>;
}

pub trait Renderer {
    fn render(&self, template: &str) -> Result<String, RenderError>;
}
