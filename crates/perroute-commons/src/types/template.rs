use serde::{Deserialize, Serialize};

use crate::template::{
    context::TemplateRenderContext, TemplateError, TemplateRender,
};

#[derive(
    Debug,
    derive_more::TryInto,
    derive_more::From,
    Clone,
    Serialize,
    Deserialize,
)]
pub enum Template {
    Sms(SmsTemplate),
    Email(EmailTemplate),
    Push(PushTemplate),
}

impl Template {
    pub fn render(
        &self,
        template_render: &dyn TemplateRender,
        context: &TemplateRenderContext,
    ) -> Result<Self, TemplateError> {
        match self {
            Template::Sms(t) => {
                t.render(template_render, context).map(Template::Sms)
            }
            Template::Email(t) => {
                t.render(template_render, context).map(Template::Email)
            }
            Template::Push(t) => {
                t.render(template_render, context).map(Template::Push)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmsTemplate {
    body: String,
}

impl SmsTemplate {
    pub fn new(body: String) -> Self {
        Self { body }
    }

    pub fn render(
        &self,
        template_render: &dyn TemplateRender,
        context: &TemplateRenderContext,
    ) -> Result<Self, TemplateError> {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailTemplate {
    pub subject: String,
    pub html: String,
    pub text: String,
}

impl EmailTemplate {
    pub fn new(subject: String, html: String, text: String) -> Self {
        Self {
            subject,
            html,
            text,
        }
    }

    pub fn render(
        &self,
        template_render: &dyn TemplateRender,
        context: &TemplateRenderContext,
    ) -> Result<Self, TemplateError> {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushTemplate {
    pub title: String,
    pub body: String,
}

impl PushTemplate {
    pub fn new(title: String, body: String) -> Self {
        Self { title, body }
    }

    pub fn render(
        &self,
        template_render: &dyn TemplateRender,
        context: &TemplateRenderContext,
    ) -> Result<Self, TemplateError> {
        todo!()
    }
}
