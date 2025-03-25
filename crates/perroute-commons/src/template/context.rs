use serde::Serialize;

use crate::types::{vars::Vars, Payload};

#[derive(Debug, Serialize)]
pub struct TemplateRenderContext<'ctx> {
    payload: &'ctx Payload,
    vars: &'ctx Vars,
}

impl<'ctx> TemplateRenderContext<'ctx> {
    pub fn new(
        payload: &'ctx Payload,
        vars: &'ctx Vars,
    ) -> TemplateRenderContext<'ctx> {
        TemplateRenderContext { payload, vars }
    }
}
