use std::fmt::Debug;

use super::{payload::Payload, recipient::Recipient, vars::Vars};
use serde::Serialize;
use sqlx::Type;

pub mod template_handlebars;

#[derive(Debug, Clone, PartialEq, Eq, Type, serde::Serialize)]
#[sqlx(transparent)]
pub struct TemplateSnippet(String);

impl TemplateSnippet {
    pub fn render<D: Serialize, T: TemplateRender<D>>(
        &self,
        template_render: &T,
        data: &D,
    ) -> Result<String, TemplateError> {
        template_render.render(self.as_ref(), data)
    }
}

impl From<String> for TemplateSnippet {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<TemplateSnippet> for String {
    fn from(value: TemplateSnippet) -> Self {
        value.0
    }
}

impl AsRef<str> for TemplateSnippet {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TemplateError {
    #[error("{0}")]
    RenderError(String),
}

pub trait TemplateRender<D: Serialize>: Debug {
    fn render(&self, template: &str, data: &D) -> Result<String, TemplateError>;
}

#[derive(Debug, Serialize)]
pub struct TemplateData {
    pub payload: Payload,
    pub recipient: Recipient,
    pub vars: Vars,
}
