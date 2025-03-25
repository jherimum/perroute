use std::marker::PhantomData;
use serde::{Deserialize, Serialize};
use crate::render::{RenderError, Renderer};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Template<S> {
    Sms(SmsTemplate<S>),
    Email(EmailTemplate<S>),
    Push(PushTemplate<S>),
}

impl Template<NotRenderedTemplateState> {
    pub fn email(
        subject: &str,
        html: &str,
        text: &str,
    ) -> Template<NotRenderedTemplateState> {
        Template::Email(EmailTemplate {
            subject: subject.to_string(),
            html: html.to_string(),
            text: text.to_string(),
            state: PhantomData::<NotRenderedTemplateState>,
        })
    }

    pub fn push(title: &str, body: &str) -> Template<NotRenderedTemplateState> {
        Template::Push(PushTemplate {
            title: title.to_string(),
            body: body.to_string(),
            state: PhantomData::<NotRenderedTemplateState>,
        })
    }

    pub fn sms(body: &str) -> Template<NotRenderedTemplateState> {
        Template::Sms(SmsTemplate {
            body: body.to_string(),
            state: PhantomData::<NotRenderedTemplateState>,
        })
    }

    pub fn render(
        &self,
        renderer: &dyn Renderer,
    ) -> Result<Template<RenderedTemplateState>, RenderError> {
        Ok(match &self {
            Template::Sms(template_data) => {
                Template::Sms(template_data.render(renderer)?)
            }
            Template::Push(template_data) => {
                Template::Push(template_data.render(renderer)?)
            }
            Template::Email(template_data) => {
                Template::Email(template_data.render(renderer)?)
            }
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotRenderedTemplateState;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderedTemplateState;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmailTemplate<S> {
    subject: String,
    html: String,
    text: String,
    #[serde(skip)]
    state: PhantomData<S>,
}

impl<S> From<EmailTemplate<S>> for Template<S> {
    fn from(value: EmailTemplate<S>) -> Self {
        Template::Email(value)
    }
}

impl EmailTemplate<NotRenderedTemplateState> {
    fn render(
        &self,
        renderer: &dyn Renderer,
    ) -> Result<EmailTemplate<RenderedTemplateState>, RenderError> {
        Ok(EmailTemplate {
            subject: renderer.render(&self.subject)?,
            html: renderer.render(&self.html)?,
            text: renderer.render(&self.text)?,
            state: PhantomData::<RenderedTemplateState>,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SmsTemplate<S> {
    body: String,
    #[serde(skip)]
    state: PhantomData<S>,
}

impl<S> From<SmsTemplate<S>> for Template<S> {
    fn from(value: SmsTemplate<S>) -> Self {
        Template::Sms(value)
    }
}

impl SmsTemplate<NotRenderedTemplateState> {
    fn render(
        &self,
        renderer: &dyn Renderer,
    ) -> Result<SmsTemplate<RenderedTemplateState>, RenderError> {
        Ok(SmsTemplate {
            body: renderer.render(&self.body)?,
            state: PhantomData::<RenderedTemplateState>,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PushTemplate<S> {
    title: String,
    body: String,
    #[serde(skip)]
    state: PhantomData<S>,
}

impl<S> From<PushTemplate<S>> for Template<S> {
    fn from(value: PushTemplate<S>) -> Self {
        Template::Push(value)
    }
}

impl PushTemplate<NotRenderedTemplateState> {
    fn render(
        &self,
        renderer: &dyn Renderer,
    ) -> Result<PushTemplate<RenderedTemplateState>, RenderError> {
        Ok(PushTemplate {
            title: renderer.render(&self.title)?,
            body: renderer.render(&self.body)?,
            state: PhantomData::<RenderedTemplateState>,
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::render::{TemplateRenderContext, TemplateRenderPlugin};

    use super::*;

    pub struct FakeTemplateRenderPlugin;

    pub struct FakeRenderer<'ctx> {
        ctx: TemplateRenderContext<'ctx>,
    }

    impl<'ctx> Renderer for FakeRenderer<'ctx> {
        fn render(&self, template: &str) -> Result<String, RenderError> {
            Ok(template.replace("a", "b").to_owned())
        }
    }

    impl TemplateRenderPlugin for FakeTemplateRenderPlugin {
        fn renderer<'c>(
            &self,
            context: TemplateRenderContext<'c>,
        ) -> Box<dyn Renderer + 'c> {
            Box::new(FakeRenderer { ctx: context })
        }
    }

    #[test]
    fn serialize() {}

    #[test]
    fn teste() {
        // let ctx = TemplateRenderContext::new(json!({}), json!({}));

        // let plugin = FakeTemplateRenderPlugin;
        // let renderer = plugin.renderer(&ctx);

        // let template = Template::email("subject a", "html_a", "text_a");
        // let result = template.render(renderer.as_ref()).unwrap();

        // assert_eq!(
        //     result,
        //     Template::Email(TemplateData {
        //         data: EmailTemplate {
        //             subject: "subject b".to_string(),
        //             html: "html_b".to_string(),
        //             text: "text_b".to_string()
        //         },
        //         state: PhantomData::<RenderedTemplateState>
        //     })
        // )
    }
}
