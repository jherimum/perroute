use serde::Serialize;

pub enum TemplateError {}

pub trait Template {
    fn render<T: Serialize>(&self, data: &T) -> String;
}

pub struct HandleBarTemplate {}
