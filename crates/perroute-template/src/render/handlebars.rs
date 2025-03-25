use std::sync::Arc;

use handlebars::Handlebars;
use super::{RenderError, Renderer, TemplateRenderContext, TemplateRenderPlugin};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("")]
    RWenderError(#[from] handlebars::RenderError),
}

#[derive(Clone)]
pub struct HandlebarsPlugin<'hb> {
    handlebars: Arc<Handlebars<'hb>>,
}

impl<'hb> HandlebarsPlugin<'hb> {
    pub fn new() -> Self {
        HandlebarsPlugin {
            handlebars: Arc::new(Handlebars::new()),
        }
    }
}

impl<'hb> TemplateRenderPlugin for HandlebarsPlugin<'hb> {
    fn renderer<'c>(
        &'c self,
        context: TemplateRenderContext<'c>,
    ) -> Box<dyn Renderer + 'c> {
        Box::new(HandlebarsRenderer {
            handlebars: &self.handlebars,
            ctx: context,
        })
    }
}

struct HandlebarsRenderer<'hb> {
    handlebars: &'hb Handlebars<'hb>,
    ctx: TemplateRenderContext<'hb>,
}

impl<'hb> Renderer for HandlebarsRenderer<'hb> {
    fn render(&self, template: &str) -> Result<String, RenderError> {
        Ok(self
            .handlebars
            .render_template(template, &self.ctx)
            .map_err(Error::from)?)
    }
}
